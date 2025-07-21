use std::future::{ready, Ready};
use actix_web::{
    body::{BoxBody, EitherBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use serde_json::json;

use crate::service::jwt_service::JwtService;

pub struct Authentication;

impl Authentication {
    pub fn new() -> Self {
        Authentication
    }
}

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Skip authentication for public routes like login and register
        if req.path() == "/api/users/login"
            || req.path() == "/api/users/register"
            || req.path() == "/api/users/verify-email"
            || req.path() == "/api/users/resend-verification"
        {
            let fut = self.service.call(req);
            return Box::pin(async move { fut.await.map(|res| res.map_into_left_body()) });
        }

        let auth_token = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "));

        let token = match auth_token {
            Some(token) => token,
            None => {
                let res = req.into_response(
                    HttpResponse::Unauthorized().json(json!({
                        "status": "error",
                        "message": "No authorization token provided"
                    })),
                );
                return Box::pin(async { Ok(res.map_into_right_body()) });
            }
        };

        // Verify the token
        let jwt_service = JwtService::new();
        match jwt_service.verify_token(token) {
            Ok(claims) => {
                req.extensions_mut().insert(claims);
                let fut = self.service.call(req);
                Box::pin(async move { fut.await.map(|res| res.map_into_left_body()) })
            }
            Err(_) => {
                let res = req.into_response(
                    HttpResponse::Unauthorized().json(json!({
                        "status": "error",
                        "message": "Invalid or expired token"
                    })),
                );
                Box::pin(async { Ok(res.map_into_right_body()) })
            }
        }
    }
} 