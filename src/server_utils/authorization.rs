use crate::api::error::ApiError;
use crate::api::jwt::{get_jwt_validation, JwtClaim, JwtKeys};
use crate::api::response::ApiResponse;
use actix_web::body::{EitherBody, MessageBody};
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Data;
use actix_web::{Error, HttpMessage, HttpResponse};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::decode;
use log::{log, Level};
use regex::Regex;
use sqlx::{query, Pool, Postgres};
use std::future::{ready, Ready};
use std::rc::Rc;
use std::sync::OnceLock;
use std::time::SystemTime;

pub struct Authorisation;

const BEARER_VALIDATION_REGEX: &str = r"Bearer (.+)";
static BEARER_REGEX: OnceLock<Regex> = OnceLock::new();
impl<S, B> Transform<S, ServiceRequest> for Authorisation
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static + MessageBody + std::fmt::Debug,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthorisationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthorisationMiddleware {
            service: service.into(),
        }))
    }
}

pub struct AuthorisationMiddleware<S> {
    // wrap with Rc to get static lifetime for async function calls in `call`
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthorisationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static + MessageBody + std::fmt::Debug,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // to use it in the closure for async function calls
        let srv = self.service.clone();
        // grab url path from request to care for 'login'
        let url_path = req.path().split("/").last().unwrap().to_owned();

        async fn authorize(
            req: &ServiceRequest,
            db_pool: &Data<Pool<Postgres>>,
        ) -> Option<ApiError> {
            let jwt_keys = match req.app_data::<Data<JwtKeys>>() {
                None => {
                    log!(Level::Error, "No JWT keys found in request: {:?}", req);
                    panic!("No JWT keys found in request: {:?}", req);
                }
                Some(keys) => keys,
            };
            let auth_header = match req.headers().get("Authorization") {
                None => {
                    log!(Level::Trace, "No Authorization header found: {:?}", req);
                    return Some(ApiError::Unauthorized);
                }
                Some(header_value) => {
                    if header_value.is_empty() {
                        log!(Level::Trace, "Empty Authorization header: {:?}", req);
                        return Some(ApiError::Unauthorized);
                    } else {
                        match header_value.to_owned().to_str() {
                            Err(_) => {
                                log!(
                                    Level::Trace,
                                    "Couldn't convert authorization header value to &str: {:?}",
                                    req
                                );
                                return Some(ApiError::Unauthorized);
                            }
                            Ok(value) => value.to_string(),
                        }
                    }
                }
            };
            let token_matcher =
                BEARER_REGEX.get_or_init(|| Regex::new(BEARER_VALIDATION_REGEX).unwrap());
            let token_capture = match token_matcher.captures(&auth_header) {
                None => {
                    log!(
                        Level::Trace,
                        "Authorization header has no Bearer token: {:?}",
                        req
                    );
                    return Some(ApiError::Unauthorized);
                }
                Some(capture) => capture,
            };
            let token = match token_capture.get(1) {
                None => {
                    log!(
                        Level::Trace,
                        "Authorization header has an empty Bearer token: {:?}",
                        req
                    );
                    return Some(ApiError::Unauthorized);
                }
                Some(capture) => capture.as_str().to_string(),
            };
            let token_decode_result =
                decode::<JwtClaim>(&token, &jwt_keys.decode_key, &get_jwt_validation());
            let session_id_claim = match token_decode_result {
                Err(_) => {
                    log!(
                        Level::Trace,
                        "Authorization header has a non valid signature or payload: {:?}",
                        req
                    );
                    return Some(ApiError::Unauthorized);
                }
                Ok(claim) => claim,
            };

            let session_id = match session_id_claim.claims.try_into_uuid() {
                Err(_) => {
                    log!(Level::Trace, "Payload is not an UUID: {:?}", req);
                    return Some(ApiError::Unauthorized);
                }
                Ok(uuid) => uuid,
            };

            // authenticate
            let session_row = match query!(
                r#"
                SELECT account_id, expires_at FROM session WHERE id = $1
                "#,
                session_id
            )
            .fetch_optional(&***db_pool)
            .await
            {
                Err(err) => {
                    log!(
                        Level::Trace,
                        "DB returned an error in authorize: \nreq: {:?} \noriginal error: {:?}",
                        req,
                        err
                    );
                    return Some(ApiError::Unauthorized);
                }
                Ok(session_id_option) => match session_id_option {
                    None => {
                        log!(Level::Trace, "No session found: \nreq: {:?}", req,);
                        return Some(ApiError::Unauthorized);
                    }
                    Some(row) => row,
                },
            };
            //check whether expired
            if session_row.expires_at.as_utc().unix_timestamp()
                < SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64
            {
                return Some(ApiError::Expired);
            }

            let account_id = session_row.account_id;
            let updated_session_row_result = query!(
                r#"
                UPDATE session SET expires_at = DEFAULT
                WHERE id = $1
                RETURNING expires_at
                "#,
                session_id
            )
            .fetch_one(&***db_pool)
            .await;

            let updated_session_row = match updated_session_row_result {
                Err(_) => {
                    log!(Level::Warn, "Couldn't update session row: \nreq: {:?}", req);
                    return Some(ApiError::Unauthorized);
                }
                Ok(row) => row,
            };

            req.extensions_mut().insert(token);
            req.extensions_mut().insert(account_id);
            req.extensions_mut()
                .insert(updated_session_row.expires_at.as_utc().unix_timestamp());

            None
        }

        Box::pin(async move {
            if !url_path.starts_with("login") {
                let db_pool = match req.app_data::<Data<Pool<Postgres>>>() {
                    None => {
                        let error_msg = "No DB pool found in request";
                        log!(Level::Error, "{}: {:?}", error_msg, req);
                        panic!("No DB pool found in request: {:?}", req);
                    }
                    Some(pool) => pool,
                };
                // remove outdated
                let _ = query!(
                    r#"
                    DELETE FROM session
                    WHERE expires_at < CURRENT_TIMESTAMP - INTERVAL '50 minutes';
                    "#
                )
                .execute(&***db_pool)
                .await
                .unwrap();

                let auth_option = authorize(&req, db_pool).await;
                match auth_option {
                    Some(err) => {
                        let new_body = ApiResponse {
                            expires_at: 0,
                            token: "".to_string(),
                            error: Some(err),
                            data: (),
                        };
                        let new_http_response = HttpResponse::Ok().json(new_body);
                        let new_service_response =
                            ServiceResponse::new(req.request().clone(), new_http_response);
                        return Ok(new_service_response.map_into_right_body());
                    }
                    None => (),
                }
            }
            //call other middleware and handler and get the response
            let res = srv.call(req).await?;
            let _request = res.request().clone();
            if !url_path.starts_with("login") {
                //log!(Level::Info, "Middleware called after server fn");
            }
            Ok(res.map_into_left_body())
        })
    }
}
