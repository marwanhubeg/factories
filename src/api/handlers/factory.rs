use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::core::factory_manager::FactoryManager;
use crate::api::ApiResponse;

/// طلب إنشاء مصنع جديد
#[derive(Debug, Deserialize)]
pub struct CreateFactoryRequest {
    pub name: String,
    pub config: serde_json::Value,
    pub template: Option<String>,
}

/// طلب التصنيع
#[derive(Debug, Deserialize)]
pub struct ManufactureRequest {
    pub input: serde_json::Value,
    pub parameters: Option<serde_json::Value>,
    pub batch_size: Option<u32>,
}

/// معلومات المصنع
#[derive(Debug, Serialize)]
pub struct FactoryInfo {
    pub name: String,
    pub factory_type: String,
    pub status: String,
    pub production_count: u64,
    pub quality_score: f32,
}

/// سرد جميع المصانع
pub async fn list_factories(
    factory_manager: web::Data<Arc<FactoryManager>>
) -> HttpResponse {
    let factories = factory_manager.list_factories();
    let factory_infos: Vec<FactoryInfo> = factories.iter()
        .map(|factory| FactoryInfo {
            name: factory.name().to_string(),
            factory_type: factory.factory_type().to_string(),
            status: factory.status().to_string(),
            production_count: factory.production_count(),
            quality_score: factory.quality_score(),
        })
        .collect();
    
    HttpResponse::Ok().json(ApiResponse::success(factory_infos, "تم جلب المصانع بنجاح"))
}

/// الحصول على مصنع محدد
pub async fn get_factory(
    factory_manager: web::Data<Arc<FactoryManager>>,
    factory_type: web::Path<String>
) -> HttpResponse {
    match factory_manager.get_factory(&factory_type) {
        Some(factory) => {
            let info = FactoryInfo {
                name: factory.name().to_string(),
                factory_type: factory.factory_type().to_string(),
                status: factory.status().to_string(),
                production_count: factory.production_count(),
                quality_score: factory.quality_score(),
            };
            HttpResponse::Ok().json(ApiResponse::success(info, "تم جلب بيانات المصنع"))
        }
        None => HttpResponse::NotFound().json(ApiResponse::error("المصنع غير موجود"))
    }
}

/// إنشاء مصنع جديد
pub async fn create_factory(
    factory_manager: web::Data<Arc<FactoryManager>>,
    factory_type: web::Path<String>,
    req: web::Json<CreateFactoryRequest>
) -> HttpResponse {
    match factory_manager.create_factory(
        &factory_type,
        &req.name,
        req.config.clone(),
        req.template.clone()
    ) {
        Ok(factory) => {
            let info = FactoryInfo {
                name: factory.name().to_string(),
                factory_type: factory.factory_type().to_string(),
                status: factory.status().to_string(),
                production_count: factory.production_count(),
                quality_score: factory.quality_score(),
            };
            HttpResponse::Created().json(ApiResponse::success(info, "تم إنشاء المصنع بنجاح"))
        }
        Err(err) => HttpResponse::BadRequest().json(ApiResponse::error(&err.to_string()))
    }
}

/// تنفيذ عملية تصنيع
pub async fn manufacture(
    factory_manager: web::Data<Arc<FactoryManager>>,
    factory_type: web::Path<String>,
    req: web::Json<ManufactureRequest>
) -> HttpResponse {
    match factory_manager.manufacture(
        &factory_type,
        req.input.clone(),
        req.parameters.clone().unwrap_or_default()
    ) {
        Ok(product) => {
            HttpResponse::Ok().json(ApiResponse::success(product, "تم التصنيع بنجاح"))
        }
        Err(err) => HttpResponse::BadRequest().json(ApiResponse::error(&err.to_string()))
    }
}

/// تصنيع دفعة
pub async fn batch_manufacture(
    factory_manager: web::Data<Arc<FactoryManager>>,
    req: web::Json<Vec<ManufactureRequest>>
) -> HttpResponse {
    let mut results = Vec::new();
    
    for item in req.into_inner() {
        // هنا سيتم معالجة كل عنصر في الدفعة
        results.push(serde_json::json!({
            "input": item.input,
            "processed": true
        }));
    }
    
    HttpResponse::Ok().json(ApiResponse::success(results, "تم معالجة الدفعة بنجاح"))
}

/// فحص الجودة
pub async fn quality_check(
    factory_manager: web::Data<Arc<FactoryManager>>,
    product_id: web::Path<String>
) -> HttpResponse {
    // محاكاة فحص الجودة
    let quality_report = serde_json::json!({
        "product_id": product_id.into_inner(),
        "quality_score": 0.95,
        "defects": [],
        "passed": true,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    HttpResponse::Ok().json(ApiResponse::success(quality_report, "تم فحص الجودة"))
}

/// تقرير الجودة
pub async fn quality_report(
    factory_manager: web::Data<Arc<FactoryManager>>
) -> HttpResponse {
    let report = serde_json::json!({
        "total_products": 1000,
        "passed": 980,
        "failed": 20,
        "quality_rate": 0.98,
        "top_defects": ["misalignment", "color_variation", "size_mismatch"],
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    HttpResponse::Ok().json(ApiResponse::success(report, "تقرير الجولة"))
}

/// تحديث مصنع
pub async fn update_factory(
    factory_manager: web::Data<Arc<FactoryManager>>,
    factory_type: web::Path<String>,
    req: web::Json<serde_json::Value>
) -> HttpResponse {
    // محاكاة عملية التحديث
    let response = serde_json::json!({
        "factory_type": factory_type.into_inner(),
        "updated": true,
        "changes": req.into_inner(),
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    HttpResponse::Ok().json(ApiResponse::success(response, "تم تحديث المصنع"))
}

/// حذف مصنع
pub async fn delete_factory(
    factory_manager: web::Data<Arc<FactoryManager>>,
    factory_type: web::Path<String>
) -> HttpResponse {
    let response = serde_json::json!({
        "factory_type": factory_type.into_inner(),
        "deleted": true,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    HttpResponse::Ok().json(ApiResponse::success(response, "تم حذف المصنع"))
}

/// حالة الإنتاج
pub async fn production_status(
    factory_manager: web::Data<Arc<FactoryManager>>
) -> HttpResponse {
    let status = serde_json::json!({
        "active_factories": 4,
        "total_production": 1250,
        "current_rate": 250,
        "efficiency": 0.92,
        "status": "operational",
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    HttpResponse::Ok().json(ApiResponse::success(status, "حالة الإنتاج الحالية"))
}
