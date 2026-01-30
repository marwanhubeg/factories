use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header;
use actix_web::{Error, HttpMessage, HttpResponse};
use futures_util::future::{ok, Ready};
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

/// هيكل الـ JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub role: String,
}

/// وسيط المصادقة
pub struct AuthMiddleware;

impl AuthMiddleware {
    pub fn new() -> Self {
        Self
    }
    
    /// توليد توكن JWT
    pub fn generate_token(user_id: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;
        
        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
            role: role.to_string(),
        };
        
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(b"marwan_hub_secret_key")
        )
    }
    
    /// التحقق من التوكن
    pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(b"marwan_hub_secret_key"),
            &Validation::default()
        ).map(|data| data.claims)
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareInner<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareInner { service })
    }
}

pub struct AuthMiddlewareInner<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareInner<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // استثناء بعض المسارات من المصادقة
        let path = req.path();
        if path == "/api/v1/system/health" || 
           path.starts_with("/api/v1/learn/") ||
           req.method() == actix_web::http::Method::OPTIONS {
            let fut = self.service.call(req);
            return Box::pin(async move { fut.await });
        }
        
        // التحقق من وجود توكن
        if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    
                    match AuthMiddleware::verify_token(token) {
                        Ok(claims) => {
                            // إضافة المطالبات إلى الطلب
                            req.extensions_mut().insert(claims);
                            let fut = self.service.call(req);
                            return Box::pin(async move { fut.await });
                        }
                        Err(_) => {
                            // التوكن غير صالح
                            let fut = async {
                                Ok(ServiceResponse::new(
                                    req.into_parts().0,
                                    HttpResponse::Unauthorized().json(
                                        crate::api::ApiResponse::<()>::error("توكن غير صالح أو منتهي الصلاحية")
                                    )
                                ))
                            };
                            return Box::pin(fut);
                        }
                    }
                }
            }
        }
        
        // لا يوجد توكن
        let fut = async {
            Ok(ServiceResponse::new(
                req.into_parts().0,
                HttpResponse::Unauthorized().json(
                    crate::api::ApiResponse::<()>::error("مصادقة مطلوبة")
                )
            ))
        };
        
        Box::pin(fut)
    }
}
