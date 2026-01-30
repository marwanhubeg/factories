use actix_web::{web, HttpResponse};
use std::sync::Arc;
use crate::core::factory_manager::FactoryManager;
use crate::api::ApiResponse;
use serde::{Deserialize, Serialize};

/// طلب توليد المحتوى
#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    pub template: String,
    pub parameters: serde_json::Value,
    pub language: Option<String>,
    pub style: Option<String>,
}

/// طلب تحليل المحتوى
#[derive(Debug, Deserialize)]
pub struct AnalyzeRequest {
    pub content: String,
    pub analysis_type: String,
    pub parameters: Option<serde_json::Value>,
}

/// نتيجة التحليل
#[derive(Debug, Serialize)]
pub struct AnalysisResult {
    pub score: f32,
    pub feedback: Vec<String>,
    pub suggestions: Vec<String>,
    pub metadata: serde_json::Value,
}

/// سرد القوالب المتاحة
pub async fn list_templates(
    factory_manager: web::Data<Arc<FactoryManager>>
) -> HttpResponse {
    let templates = vec![
        "website_template",
        "course_structure", 
        "document_template",
        "design_system",
        "business_plan",
        "technical_doc",
        "creative_writing",
        "code_template"
    ];
    
    HttpResponse::Ok().json(ApiResponse::success(templates, "القوالب المتاحة"))
}

/// توليد محتوى بناءً على قالب
pub async fn generate_content(
    factory_manager: web::Data<Arc<FactoryManager>>,
    req: web::Json<GenerateRequest>
) -> HttpResponse {
    // محاكاة توليد المحتوى
    let generated_content = serde_json::json!({
        "template": req.template,
        "content": format!("محتوى مولد بناءً على قالب: {} مع المعلمات: {}", 
            req.template, req.parameters),
        "language": req.language.clone().unwrap_or("ar".to_string()),
        "style": req.style.clone().unwrap_or("professional".to_string()),
        "length": 500,
        "quality_score": 0.92,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    HttpResponse::Ok().json(ApiResponse::success(generated_content, "تم توليد المحتوى"))
}

/// تحليل المحتوى
pub async fn analyze_content(
    factory_manager: web::Data<Arc<FactoryManager>>,
    req: web::Json<AnalyzeRequest>
) -> HttpResponse {
    let analysis_result = AnalysisResult {
        score: 0.88,
        feedback: vec![
            "المحتوى جيد من حيث التنظيم".to_string(),
            "يمكن تحسين الصياغة".to_string(),
            "الأفكار الرئيسية واضحة".to_string()
        ],
        suggestions: vec![
            "إضافة أمثلة توضيحية".to_string(),
            "تحسين العناوين".to_string(),
            "إضافة روابط مرجعية".to_string()
        ],
        metadata: serde_json::json!({
            "word_count": req.content.split_whitespace().count(),
            "readability_score": 0.75,
            "top_keywords": ["تعليم", "تقنية", "إبداع"],
            "sentiment": "positive"
        })
    };
    
    HttpResponse::Ok().json(ApiResponse::success(analysis_result, "تم تحليل المحتوى"))
}
