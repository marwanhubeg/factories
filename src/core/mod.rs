//! النواة الأساسية لنظام مصانع مروان هوب
//! تحتوي على المكونات الأساسية المشتركة بين جميع المصانع

pub mod factory;
pub mod quality;
pub mod config;
pub mod errors;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// الأنواع الأساسية للمصانع
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FactoryType {
    Education,
    Creative,
    Corporate,
    Technology,
}

impl FactoryType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FactoryType::Education => "education",
            FactoryType::Creative => "creative",
            FactoryType::Corporate => "corporate",
            FactoryType::Technology => "technology",
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "education" | "edu" => Some(FactoryType::Education),
            "creative" | "cre" => Some(FactoryType::Creative),
            "corporate" | "corp" => Some(FactoryType::Corporate),
            "technology" | "tech" => Some(FactoryType::Technology),
            _ => None,
        }
    }
}

/// حالة النظام
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    pub version: String,
    pub mhos_version: String,
    pub factories: HashMap<String, FactoryState>,
    pub uptime: u64,
    pub last_updated: DateTime<Utc>,
    pub metrics: SystemMetrics,
}

/// مقاييس النظام
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub total_projects: u64,
    pub successful_projects: u64,
    pub failed_projects: u64,
    pub avg_processing_time: f64,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub active_connections: u32,
}

/// تهيئة النظام الأساسي
pub fn initialize_core() -> Result<(), String> {
    log::info!("تهيئة النواة الأساسية...");
    
    // التحقق من المتطلبات الأساسية
    check_prerequisites()?;
    
    // تهيئة التسجيل
    initialize_logging();
    
    // تهيئة التكوين
    initialize_config();
    
    log::info!("✅ تم تهيئة النواة الأساسية بنجاح");
    Ok(())
}

/// التحقق من المتطلبات الأساسية
fn check_prerequisites() -> Result<(), String> {
    // التحقق من إمكانية الوصول إلى الملفات
    let required_dirs = ["data", "templates", "outputs", "logs"];
    for dir in required_dirs {
        if !std::path::Path::new(dir).exists() {
            std::fs::create_dir_all(dir)
                .map_err(|e| format!("فشل في إنشاء المجلد {}: {}", dir, e))?;
        }
    }
    
    Ok(())
}

/// تهيئة نظام التسجيل
fn initialize_logging() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .format_module_path(false)
        .init();
}

/// تهيئة التكوين
fn initialize_config() {
    // سيتم تحميل التكوين من ملف config.toml
    log::info!("📋 جاري تحميل التكوين...");
}

/// خاصية أساسية لكل مكون في النظام
pub trait CoreComponent: Send + Sync {
    fn get_name(&self) -> String;
    fn get_version(&self) -> String;
    fn initialize(&self) -> Result<(), String>;
    fn shutdown(&self) -> Result<(), String>;
    fn get_status(&self) -> ComponentStatus;
}

/// حالة المكون
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComponentStatus {
    Initializing,
    Ready,
    Running,
    Warning,
    Error,
    Shutdown,
}

impl ComponentStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ComponentStatus::Initializing => "initializing",
            ComponentStatus::Ready => "ready",
            ComponentStatus::Running => "running",
            ComponentStatus::Warning => "warning",
            ComponentStatus::Error => "error",
            ComponentStatus::Shutdown => "shutdown",
        }
    }
}

/// نتيجة العملية
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub execution_time_ms: u64,
    pub timestamp: DateTime<Utc>,
}

impl<T> OperationResult<T> {
    pub fn success(data: T, execution_time_ms: u64) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            execution_time_ms,
            timestamp: Utc::now(),
        }
    }
    
    pub fn error(error: String, execution_time_ms: u64) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            execution_time_ms,
            timestamp: Utc::now(),
        }
    }
}

/// مولد معرف فريد
pub fn generate_id(prefix: &str) -> String {
    format!("{}_{}", prefix, uuid::Uuid::new_v4().to_string()[..8].to_string())
}

/// تنسيق التاريخ العربي
pub fn format_arabic_date(dt: DateTime<Utc>) -> String {
    use chrono::Datelike;
    
    let hijri_year = dt.year() - 579; // تقريب للهجري
    let months_ar = [
        "محرم", "صفر", "ربيع الأول", "ربيع الثاني",
        "جمادى الأولى", "جمادى الآخرة", "رجب", "شعبان",
        "رمضان", "شوال", "ذو القعدة", "ذو الحجة"
    ];
    
    let month_ar = months_ar[(dt.month() as usize - 1) % 12];
    
    format!("{} {} {} هـ", dt.day(), month_ar, hijri_year)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_factory_type_conversion() {
        assert_eq!(FactoryType::Education.as_str(), "education");
        assert_eq!(FactoryType::from_str("education"), Some(FactoryType::Education));
        assert_eq!(FactoryType::from_str("EDUCATION"), Some(FactoryType::Education));
        assert_eq!(FactoryType::from_str("unknown"), None);
    }
    
    #[test]
    fn test_generate_id() {
        let id = generate_id("test");
        assert!(id.starts_with("test_"));
        assert_eq!(id.len(), "test_xxxxxxxx".len());
    }
    
    #[test]
    fn test_operation_result() {
        let success = OperationResult::success("test", 100);
        assert!(success.success);
        assert_eq!(success.data, Some("test"));
        assert!(success.error.is_none());
        
        let error = OperationResult::<String>::error("test error".to_string(), 50);
        assert!(!error.success);
        assert!(error.data.is_none());
        assert_eq!(error.error, Some("test error".to_string()));
    }
}
