pub mod handlers;
pub mod middleware;
pub mod routes;

use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use crate::core::factory_manager::FactoryManager;

/// خادم API الرئيسي للنظام
pub struct ApiServer {
    factory_manager: Arc<FactoryManager>,
    port: u16,
}

impl ApiServer {
    /// إنشاء مثيل جديد لخادم API
    pub fn new(factory_manager: Arc<FactoryManager>, port: u16) -> Self {
        Self {
            factory_manager,
            port,
        }
    }

    /// تشغيل خادم API
    pub async fn run(&self) -> std::io::Result<()> {
        let factory_manager = self.factory_manager.clone();
        let port = self.port;
        
        log::info!("🚀 بدء تشغيل خادم API على المنفذ {}", port);
        
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(factory_manager.clone()))
                .configure(routes::configure)
                .wrap(middleware::CorsMiddleware::new())
                .wrap(middleware::LoggerMiddleware::new())
                .wrap(middleware::AuthMiddleware::new())
        })
        .bind(("0.0.0.0", port))?
        .run()
        .await
    }
}

/// هيكل استجابة API القياسي
#[derive(serde::Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: &str) -> Self {
        Self {
            success: true,
            message: message.to_string(),
            data: Some(data),
            timestamp: chrono::Utc::now(),
        }
    }
    
    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            message: message.to_string(),
            data: None,
            timestamp: chrono::Utc::now(),
        }
    }
}
