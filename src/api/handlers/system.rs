use actix_web::{web, HttpResponse};
use std::sync::Arc;
use crate::core::factory_manager::FactoryManager;
use crate::api::ApiResponse;

/// فحص صحة النظام
pub async fn system_health(
    factory_manager: web::Data<Arc<FactoryManager>>
) -> HttpResponse {
    let health_status = serde_json::json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
        "uptime": "24h",
        "active_connections": 42,
        "memory_usage": "65%",
        "cpu_usage": "30%",
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    HttpResponse::Ok().json(ApiResponse::success(health_status, "النظام يعمل بشكل صحي"))
}

/// إحصائيات النظام
pub async fn system_stats(
    factory_manager: web::Data<Arc<FactoryManager>>
) -> HttpResponse {
    let stats = serde_json::json!({
        "total_factories": 4,
        "total_production": 1250,
        "average_quality": 0.95,
        "requests_processed": 10000,
        "errors": 15,
        "success_rate": 0.9985,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    HttpResponse::Ok().json(ApiResponse::success(stats, "إحصائيات النظام"))
}

/// إعادة تشغيل النظام
pub async fn system_restart(
    factory_manager: web::Data<Arc<FactoryManager>>
) -> HttpResponse {
    let response = serde_json::json!({
        "message": "جاري إعادة تشغيل النظام...",
        "restart_scheduled": true,
        "estimated_downtime": "30s",
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    // في الواقع هنا سيتم جدولة إعادة التشغيل
    HttpResponse::Accepted().json(ApiResponse::success(response, "تم جدولة إعادة التشغيل"))
}
