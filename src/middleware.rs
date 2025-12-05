use server_fn::middleware::{BoxedService, Layer};

pub struct AuthorisationLayer;

impl<Req,Res> Layer<Req, Res> for AuthorisationLayer {
    fn layer(&self, inner: BoxedService<Req, Res>) -> BoxedService<Req, Res> {
        inner
        //AuthorisationService { ser, service }
    }
}

pub struct AuthorisationService;

// impl<Req,Res> run for AuthorisationService {
//
// }



// use actix_web::body::{EitherBody, MessageBody};
// use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
// use actix_web::Error;
// use std::future::{ready, Ready};
// use std::rc::Rc;
// use futures_util::future::LocalBoxFuture;
//
// //TODO rewrite as leptos middleware
// pub struct Authorisation;
// impl<S, B> Transform<S, ServiceRequest> for Authorisation
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
//     S::Future: 'static,
//     B: 'static + MessageBody + std::fmt::Debug,
// {
//     type Response = ServiceResponse<EitherBody<B>>;
//     type Error = Error;
//     type Transform = AuthorisationMiddleware<S>;
//     type InitError = ();
//     type Future = Ready<Result<Self::Transform, Self::InitError>>;
//
//     fn new_transform(&self, service: S) -> Self::Future {
//         ready(Ok(AuthorisationMiddleware {
//             service: service.into(),
//         }))
//     }
// }
//
// pub struct AuthorisationMiddleware<S> {
//     // wrap with Rc to get static lifetime for async function calls in `call`
//     service: Rc<S>,
// }
//
// impl<S, B> Service<ServiceRequest> for AuthorisationMiddleware<S>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
//     S::Future: 'static,
//     B: 'static + MessageBody + std::fmt::Debug,
// {
//     type Response = ServiceResponse<EitherBody<B>>;
//     type Error = Error;
//     type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
//
//     forward_ready!(service);
//
//     fn call(&self, req: ServiceRequest) -> Self::Future {
//         todo!()
//     }
// }
