//! تكوين النظام
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// تكوين النظام
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    /// اسم النظام
    pub name: String,
    
    /// الإصدار
    pub version: String,
    
    /// بيئة التشغيل
    pub environment: Environment,
    
    /// دليل البيانات
    pub data_dir: PathBuf,
    
    /// التخزين المؤقت
    pub cache: CacheConfig,
    
    /// الأمان
    pub security: SecurityConfig,
    
    /// التسجيل
    pub logging: LoggingConfig,
}

/// بيئة التشغيل
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

/// تكوين التخزين المؤقت
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// نوع التخزين المؤقت
    pub cache_type: CacheType,
    
    /// عنوان Redis (إذا كان النوع redis)
    pub redis_url: Option<String>,
    
    /// TTL بالثواني
    pub ttl: u64,
}

/// نوع التخزين المؤقت
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheType {
    Memory,
    Redis,
    File,
}

/// تكوين الأمان
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// مفتاح JWT
    pub jwt_secret: String,
    
    /// تفعيل المصادقة
    pub enable_auth: bool,
    
    /// الحد الأقصى للطلبات
    pub rate_limit: u32,
    
    /// نافذة الوقت للحد
    pub rate_window: u64,
}

/// تكوين التسجيل
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// مستوى التسجيل
    pub level: LogLevel,
    
    /// ملف السجل
    pub file: Option<PathBuf>,
    
    /// الحجم الأقصى للملف (MB)
    pub max_size: u64,
    
    /// عدد الملفات الأقصى
    pub max_files: u32,
}

/// مستوى التسجيل
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            name: "Marwan Hub Factories".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: Environment::Development,
            data_dir: PathBuf::from("./data"),
            cache: CacheConfig {
                cache_type: CacheType::Memory,
                redis_url: None,
                ttl: 3600,
            },
            security: SecurityConfig {
                jwt_secret: "change_this_in_production".to_string(),
                enable_auth: true,
                rate_limit: 100,
                rate_window: 60,
            },
            logging: LoggingConfig {
                level: LogLevel::Info,
                file: None,
                max_size: 100,
                max_files: 10,
            },
        }
    }
}

/// تحميل التكوين
pub fn load_config() -> Result<SystemConfig, Box<dyn std::error::Error>> {
    // أولاً حاول تحميل من ملف
    let config_path = std::env::current_dir()?.join("config.toml");
    
    if config_path.exists() {
        let config_content = std::fs::read_to_string(&config_path)?;
        let config: SystemConfig = toml::from_str(&config_content)?;
        return Ok(config);
    }
    
    // إذا لم يوجد ملف، استخدم الإعدادات الافتراضية
    Ok(SystemConfig::default())
}
