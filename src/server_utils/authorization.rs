use actix_web::body::{EitherBody, MessageBody};
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Data;
use actix_web::{Error, HttpMessage};
use base64::Engine;
use bytes::Bytes;
use futures_util::future::LocalBoxFuture;
use sqlx::types::Uuid;
use sqlx::{query, Pool, Postgres};
use std::future::{ready, Ready};
use std::rc::Rc;

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
        let url_path = req.path().split("/").last().unwrap().to_owned();

        async fn authorize(req: &ServiceRequest) -> Result<i64, Error> {
            let session_secret = req
                .app_data::<Data<Bytes>>()
                .expect("No session secret from server");
            let db_pool = req
                .app_data::<Data<Pool<Postgres>>>()
                .expect("No db pool from server");
            let auth_header = match req.headers().get("Authorization") {
                None => "None".to_string(),
                Some(header_value) => {
                    if header_value.is_empty() {
                        "Empty".to_string()
                    } else {
                        header_value.to_owned().to_str().unwrap().to_string()
                    }
                }
            };
            let token = auth_header.replace("Bearer ", "");
            let token_bytes = base64::engine::general_purpose::URL_SAFE
                .decode(&token)
                .unwrap();
            let session_id = Uuid::from_slice(
                &*simple_crypt::decrypt(token_bytes.as_ref(), &session_secret).unwrap(),
            )
            .unwrap();
            // authenticate
            let session_row = query!(
                r#"
                SELECT * FROM session WHERE id = $1
                "#,
                session_id
            )
            .fetch_optional(&***db_pool)
            .await
            .unwrap();
            let expires_at = session_row.unwrap().expires_at.as_utc().unix_timestamp();
            req.extensions_mut().insert(token);
            req.extensions_mut().insert(session_id);
            req.extensions_mut().insert(expires_at);

            Ok(0)
        }

        Box::pin(async move {
            if !url_path.starts_with("login") {
                //log!(Level::Info, "Middleware called before server fn");
                let _ = authorize(&req).await.unwrap();
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
