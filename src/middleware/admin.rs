use actix_web::{
    dev::ServiceRequest, Error, error::ErrorUnauthorized,
    http::header::{self},
};
use futures_util::future::{ok, Ready};
use actix_web::dev::{Service, Transform};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::service::jwt_service::{JwtService, Role};

pub struct AdminAuthentication;

impl AdminAuthentication {
    pub fn new() -> Self {
        AdminAuthentication
    }
}

impl<S, B> Transform<S, ServiceRequest> for AdminAuthentication
where
    S: Service<ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AdminAuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AdminAuthenticationMiddleware { service })
    }
}

pub struct AdminAuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AdminAuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get(header::AUTHORIZATION);
        
        let auth_header = match auth_header {
            Some(header) => header,
            None => {
                return Box::pin(async move {
                    Err(ErrorUnauthorized("No authorization header"))
                });
            }
        };

        let auth_str = match auth_header.to_str() {
            Ok(str) => str,
            Err(_) => {
                return Box::pin(async move {
                    Err(ErrorUnauthorized("Invalid authorization header"))
                });
            }
        };

        let token = match auth_str.strip_prefix("Bearer ") {
            Some(token) => token,
            None => {
                return Box::pin(async move {
                    Err(ErrorUnauthorized("Invalid authorization format"))
                });
            }
        };

        let jwt_service = JwtService::new();
        let claims = match jwt_service.verify_token(token) {
            Ok(claims) => claims,
            Err(_) => {
                return Box::pin(async move {
                    Err(ErrorUnauthorized("Invalid token"))
                });
            }
        };

        // Check if the user has admin role
        match claims.role {
            Role::Admin => {
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            _ => Box::pin(async move {
                Err(ErrorUnauthorized("Insufficient permissions"))
            }),
        }
    }
}
