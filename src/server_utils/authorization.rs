use crate::api::error::ApiError;
use crate::api::jwt::{get_jwt_validation, JwtClaim, JwtKeys};
use crate::api::response::ApiResponse;
use actix_web::body::{EitherBody, MessageBody};
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::{Data};
use actix_web::{http, Error, HttpMessage, HttpResponse};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::decode;
use log::{log, Level};
use sqlx::{query, Pool, Postgres};
use std::future::{ready, Ready};
use std::rc::Rc;
use chrono::Utc;

/// This adds authorization to a leptos server fn.
/// # Example
/// In a leptos server fn:
/// ```
///#[server(client = crate::client::AddAuthHeaderClient)]
/// pub async fn foo() -> Result<ApiResponse<User>, ServerFnError> {
///     let req: actix_web::HttpRequest = extract().await?;
///     // use fields as required
///     let account_id = req.extensions_mut().get::<Uuid>().unwrap().clone();
///     let token = req.extensions_mut().get::<String>().unwrap().to_string();
///     let expires_at = req.extensions_mut().get::<i64>().unwrap().clone();
/// ```
pub struct Authorisation;

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
        let is_login = req.path().starts_with("/api/login");

        async fn authorize(req: &ServiceRequest, db_pool: &Pool<Postgres>) -> Option<ApiError> {
            let jwt_keys = match req.app_data::<Data<JwtKeys>>() {
                None => {
                    log!(Level::Error, "{}", get_info(req, "No JWT keys found in request context: ".to_string()));
                    return Some(ApiError::UnexpectedError(format!(
                        "Error time: {}", Utc::now(),
                    )));
                }
                Some(keys) => keys,
            };
            let auth_header = match req.headers().get("Authorization") {
                None => {
                    log!(Level::Trace, "{}", get_info(req, "No Authorization header: ".to_string()));
                    return Some(ApiError::Unauthorized);
                }
                Some(header_value) => {
                    if header_value.is_empty() {
                        log!(Level::Trace, "{}", get_info(req, "Empty Authorization header: ".to_string()));
                        return Some(ApiError::Unauthorized);
                    } else {
                        match header_value.to_str() {
                            Err(_) => {
                                log!(
                                    Level::Trace,
                                    "Couldn't convert authorization header value to &str: {}",
                                    get_info(req, "Connection Info: ".to_string())
                                );
                                return Some(ApiError::Unauthorized);
                            }
                            Ok(value) => value,
                        }
                    }
                }
            };
            let bearer_matcher = "Bearer ";
            let token = if !auth_header.starts_with(bearer_matcher) {
                log!(
                    Level::Trace, "{}",
                    get_info(req, "Authorization header has no Bearer token: ".to_string())
                );
                return Some(ApiError::Unauthorized);
            } else {
                &auth_header[bearer_matcher.len()..]
            };
            let token_decode_result =
                decode::<JwtClaim>(&token, &jwt_keys.decode_key, &get_jwt_validation());
            let session_id_claim = match token_decode_result {
                Err(_) => {
                    log!(
                        Level::Trace, "{}",
                        get_info(req, "Authorization header has a non valid signature or payload: ".to_string())
                    );
                    return Some(ApiError::Unauthorized);
                }
                Ok(claim) => claim,
            };
            let session_id = match session_id_claim.claims.try_into_uuid() {
                Err(_) => {
                    log!(
                        Level::Trace, "{}",
                        get_info(req, "Payload is not an UUID: ".to_string())
                    );
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
            .fetch_optional(db_pool)
            .await
            {
                Err(err) => {
                    log!(
                        Level::Trace,
                        "DB returned an error in authorize: \n\tReq Info: {} \n\tOriginal Error: {}",
                        get_info(req, "".to_string()),
                        err
                    );
                    return Some(ApiError::Unauthorized);
                }
                Ok(session_id_option) => match session_id_option {
                    None => {
                        log!(Level::Warn, "{}", get_info(req, "No session found:".to_string()));
                        return Some(ApiError::Unauthorized);
                    }
                    Some(row) => row,
                },
            };
            // check whether expired
            if session_row.expires_at.and_utc().timestamp() < Utc::now().timestamp() {
                return Some(ApiError::Expired);
            }
            // now we know the session is authenticated and not expired, so update session
            let account_id = session_row.account_id;
            let updated_session_row_result = query!(
                r#"
                UPDATE session SET expires_at = DEFAULT
                WHERE id = $1
                RETURNING expires_at
                "#,
                session_id
            )
            .fetch_one(db_pool)
            .await;
            let updated_session_row = match updated_session_row_result {
                Err(_) => {
                    log!(Level::Warn, "{}", get_info(req, "Could not update session row".to_string()));
                    return Some(ApiError::Unauthorized);
                }
                Ok(row) => row,
            };

            req.extensions_mut().insert(token.to_string());
            req.extensions_mut().insert(account_id);
            req.extensions_mut()
                .insert(updated_session_row.expires_at.and_utc().timestamp());

            None
        }

        Box::pin(async move {
            if !is_login {
                let db_pool = match req.app_data::<Data<Pool<Postgres>>>() {
                    None => {
                        let error_msg = "No DB pool found in request";
                        log!(Level::Error, "{}: {}", error_msg, get_info(&req, "Request Info".to_string()));
                        let new_body = ApiResponse {
                            expires_at: 0,
                            token: "".to_string(),
                            error: Some(ApiError::DBConnectionError),
                            data: (),
                        };
                        let new_http_response = HttpResponse::Ok().json(new_body);
                        let new_service_response =
                            ServiceResponse::new(req.request().clone(), new_http_response);

                        return Ok(new_service_response.map_into_right_body());
                    }
                    Some(pool) => pool.get_ref(),
                };

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
            // call other middleware and handler and get the response
            let res = srv.call(req).await?;
            let _request = res.request().clone();
            if !is_login {
                // add code here if it is to be called after the server fn
            }
            Ok(res.map_into_left_body())
        })
    }
}

fn get_info(req: &ServiceRequest, msg: String) -> String {
    let header = match req.headers().get(http::header::AUTHORIZATION) {
        None => "Missing".to_string(),
        Some(header) => {
            header.to_str().unwrap_or_else(|_| "Couldn't convert to string").to_string()
        }
    };
    let ip = match req.peer_addr() {
        None => "Not found in connection".to_string(),
        Some(ip) => ip.to_string()
    };

    format!("{} \n\tAuthorization header: {}\n\tIP:{}", msg, header, ip)
}
