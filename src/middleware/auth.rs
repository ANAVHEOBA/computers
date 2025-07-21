use std::future::{ready, Ready};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use serde_json::json;
use std::task::{Context, Poll};

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
    type Response = ServiceResponse<B>;
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
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Skip authentication for login and register routes
        if req.path() == "/api/users/login" 
            || req.path() == "/api/users/register"
            || req.path() == "/api/users/verify-email"
            || req.path() == "/api/users/resend-verification" {
            let fut = self.service.call(req);
            return Box::pin(async move { fut.await });
        }

        // Get the Authorization header
        let auth_header = req.headers().get("Authorization");
        let auth_token = match auth_header {
            Some(header) => {
                let header_str = header.to_str().unwrap_or("");
                if header_str.starts_with("Bearer ") {
                    &header_str[7..]
                } else {
                    ""
                }
            }
            None => "",
        };

        if auth_token.is_empty() {
            return Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(json!({
                            "status": "error",
                            "message": "No authorization token provided"
                        }))
                        .into_body(),
                ))
            });
        }

        // Verify the token
        let jwt_service = JwtService::new();
        match jwt_service.verify_token(auth_token) {
            Ok(claims) => {
                // Add the verified claims to the request extensions
                req.extensions_mut().insert(claims);
                let fut = self.service.call(req);
                Box::pin(async move { fut.await })
            }
            Err(_) => Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(json!({
                            "status": "error",
                            "message": "Invalid or expired token"
                        }))
                        .into_body(),
                ))
            }),
        }
    }
} 