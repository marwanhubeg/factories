use actix_web::web;
use crate::api::handlers;

/// تكوين جميع مسارات API
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // مسارات المصانع
            .route("/factories", web::get().to(handlers::list_factories))
            .route("/factories/{factory_type}", web::get().to(handlers::get_factory))
            .route("/factories/{factory_type}/create", web::post().to(handlers::create_factory))
            .route("/factories/{factory_type}/update", web::put().to(handlers::update_factory))
            .route("/factories/{factory_type}/delete", web::delete().to(handlers::delete_factory))
            
            // مسارات التصنيع
            .route("/manufacture/{factory_type}", web::post().to(handlers::manufacture))
            .route("/batch-manufacture", web::post().to(handlers::batch_manufacture))
            .route("/production/status", web::get().to(handlers::production_status))
            
            // مسارات الجودة
            .route("/quality/check/{product_id}", web::get().to(handlers::quality_check))
            .route("/quality/report", web::get().to(handlers::quality_report))
            
            // مسارات النظام
            .route("/system/health", web::get().to(handlers::system_health))
            .route("/system/stats", web::get().to(handlers::system_stats))
            .route("/system/restart", web::post().to(handlers::system_restart))
            
            // مسارات MH-OS
            .route("/mhos/dashboard", web::get().to(handlers::mhos_dashboard))
            .route("/mhos/quality-gates", web::get().to(handlers::mhos_quality_gates))
            .route("/mhos/optimize", web::post().to(handlers::mhos_optimize))
            
            // مسارات التعلم
            .route("/learn/templates", web::get().to(handlers::list_templates))
            .route("/learn/generate", web::post().to(handlers::generate_content))
            .route("/learn/analyze", web::post().to(handlers::analyze_content))
    );
}

/// تعريفات المسارات للتوثيق
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        handlers::list_factories,
        handlers::get_factory,
        handlers::create_factory,
        handlers::manufacture,
        handlers::system_health,
        handlers::mhos_dashboard,
        handlers::learn_generate
    ),
    components(
        schemas(
            handlers::CreateFactoryRequest,
            handlers::ManufactureRequest,
            handlers::ApiResponse,
            handlers::FactoryInfo
        )
    ),
    tags(
        (name = "factories", description = "إدارة المصانع"),
        (name = "manufacturing", description = "عمليات التصنيع"),
        (name = "mhos", description = "نظام MH-OS"),
        (name = "learning", description = "التعلم والتحليل")
    )
)]
pub struct ApiDoc;
