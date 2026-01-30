//! تعريف المصانع الأساسية وواجهاتها

use crate::core::quality::QualityGate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// واجهة أساسية لكل مصنع
pub trait Factory: Send + Sync {
    /// اسم المصنع
    fn get_name(&self) -> String;
    
    /// نوع المصنع
    fn get_type(&self) -> FactoryType;
    
    /// إصدار المصنع
    fn get_version(&self) -> String;
    
    /// الحالة الحالية للمصنع
    fn get_status(&self) -> FactoryStatus;
    
    /// الإمكانيات المتاحة
    fn get_capabilities(&self) -> Vec<FactoryCapability>;
    
    /// معالجة طلب
    fn process_request(&self, request: FactoryRequest) -> Result<FactoryResponse, FactoryError>;
    
    /// التحقق من جودة المخرجات
    fn validate_output(&self, output: &FactoryOutput) -> Vec<QualityGate>;
    
    /// الحصول على إحصائيات المصنع
    fn get_metrics(&self) -> FactoryMetrics;
    
    /// إعادة تعيين المصنع
    fn reset(&self) -> Result<(), FactoryError>;
}

/// نوع المصنع
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FactoryType {
    Education,
    Creative,
    Corporate,
    Technology,
}

impl FactoryType {
    pub fn display_name(&self) -> &'static str {
        match self {
            FactoryType::Education => "مصنع التعليم",
            FactoryType::Creative => "مصنع الإبداع",
            FactoryType::Corporate => "مصنع المؤسسات",
            FactoryType::Technology => "مصنع التقنية",
        }
    }
    
    pub fn description(&self) -> &'static str {
        match self {
            FactoryType::Education => "إنشاء المحتوى التعليمي والتدريبي الذكي",
            FactoryType::Creative => "التصميم والمحتوى الإبداعي والجرافيك",
            FactoryType::Corporate => "حلول المؤسسات والشركات والإدارة",
            FactoryType::Technology => "التطوير البرمجي والحلول التقنية",
        }
    }
}

/// حالة المصنع
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FactoryStatus {
    Initializing,
    Ready,
    Processing,
    Paused,
    Error(String),
    Maintenance,
    Shutdown,
}

impl FactoryStatus {
    pub fn is_operational(&self) -> bool {
        matches!(self, FactoryStatus::Ready | FactoryStatus::Processing)
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            FactoryStatus::Initializing => "initializing",
            FactoryStatus::Ready => "ready",
            FactoryStatus::Processing => "processing",
            FactoryStatus::Paused => "paused",
            FactoryStatus::Error(_) => "error",
            FactoryStatus::Maintenance => "maintenance",
            FactoryStatus::Shutdown => "shutdown",
        }
    }
}

/// إمكانية المصنع
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactoryCapability {
    pub name: String,
    pub description: String,
    pub version: String,
    pub enabled: bool,
    pub parameters: HashMap<String, CapabilityParameter>,
}

/// معامل الإمكانية
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityParameter {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub data_type: ParameterType,
    pub default_value: Option<serde_json::Value>,
}

/// نوع المعامل
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterType {
    String,
    Integer,
    Float,
    Boolean,
    Array,
    Object,
}

/// طلب المصنع
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactoryRequest {
    pub request_id: String,
    pub factory_type: FactoryType,
    pub operation: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub priority: RequestPriority,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}

/// أولوية الطلب
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl RequestPriority {
    pub fn value(&self) -> u8 {
        match self {
            RequestPriority::Low => 1,
            RequestPriority::Normal => 2,
            RequestPriority::High => 3,
            RequestPriority::Critical => 4,
        }
    }
}

/// استجابة المصنع
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactoryResponse {
    pub request_id: String,
    pub success: bool,
    pub output: Option<FactoryOutput>,
    pub error_message: Option<String>,
    pub processing_time_ms: u64,
    pub quality_score: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
}

/// مخرجات المصنع
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactoryOutput {
    pub output_type: OutputType,
    pub content: serde_json::Value,
    pub format: OutputFormat,
    pub size_bytes: usize,
    pub metadata: HashMap<String, String>,
    pub files: Vec<OutputFile>,
}

/// نوع المخرجات
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputType {
    Text,
    Document,
    Code,
    Design,
    Data,
    Media,
}

/// تنسيق المخرجات
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Markdown,
    Html,
    Json,
    Yaml,
    Pdf,
    Image,
    Video,
    Audio,
    Archive,
}

/// ملف المخرجات
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputFile {
    pub filename: String,
    pub content_type: String,
    pub content: Vec<u8>,
    pub size_bytes: usize,
    pub checksum: String,
}

/// أخطاء المصنع
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FactoryError {
    InitializationFailed(String),
    InvalidRequest(String),
    ProcessingFailed(String),
    ResourceUnavailable(String),
    ValidationFailed(String),
    Timeout,
    Unknown(String),
}

impl std::fmt::Display for FactoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FactoryError::InitializationFailed(msg) => write!(f, "فشل التهيئة: {}", msg),
            FactoryError::InvalidRequest(msg) => write!(f, "طلب غير صالح: {}", msg),
            FactoryError::ProcessingFailed(msg) => write!(f, "فشل المعالجة: {}", msg),
            FactoryError::ResourceUnavailable(msg) => write!(f, "مورد غير متاح: {}", msg),
            FactoryError::ValidationFailed(msg) => write!(f, "فشل التحقق: {}", msg),
            FactoryError::Timeout => write!(f, "انتهى وقت الانتظار"),
            FactoryError::Unknown(msg) => write!(f, "خطأ غير معروف: {}", msg),
        }
    }
}

impl std::error::Error for FactoryError {}

/// إحصائيات المصنع
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactoryMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub avg_processing_time_ms: f64,
    pub current_queue_size: usize,
    pub memory_usage_mb: f64,
    pub last_reset: DateTime<Utc>,
}

/// سجل المصانع
pub struct FactoryRegistry {
    factories: HashMap<String, Box<dyn Factory>>,
}

impl FactoryRegistry {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }
    
    pub fn register_factory(&mut self, name: String, factory: Box<dyn Factory>) {
        self.factories.insert(name, factory);
    }
    
    pub fn get_factory(&self, name: &str) -> Option<&dyn Factory> {
        self.factories.get(name).map(|f| f.as_ref())
    }
    
    pub fn list_factories(&self) -> Vec<FactoryInfo> {
        self.factories
            .iter()
            .map(|(name, factory)| FactoryInfo {
                name: name.clone(),
                factory_type: factory.get_type(),
                status: factory.get_status(),
                version: factory.get_version(),
                capabilities: factory.get_capabilities(),
                metrics: factory.get_metrics(),
            })
            .collect()
    }
}

/// معلومات المصنع
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactoryInfo {
    pub name: String,
    pub factory_type: FactoryType,
    pub status: FactoryStatus,
    pub version: String,
    pub capabilities: Vec<FactoryCapability>,
    pub metrics: FactoryMetrics,
}

/// منشئ المصانع
pub struct FactoryBuilder {
    factory_type: FactoryType,
    capabilities: Vec<FactoryCapability>,
    config: HashMap<String, serde_json::Value>,
}

impl FactoryBuilder {
    pub fn new(factory_type: FactoryType) -> Self {
        Self {
            factory_type,
            capabilities: Vec::new(),
            config: HashMap::new(),
        }
    }
    
    pub fn add_capability(mut self, capability: FactoryCapability) -> Self {
        self.capabilities.push(capability);
        self
    }
    
    pub fn set_config(mut self, key: String, value: serde_json::Value) -> Self {
        self.config.insert(key, value);
        self
    }
    
    pub fn build(self) -> Result<Box<dyn Factory>, FactoryError> {
        match self.factory_type {
            FactoryType::Education => {
                // سيتم تعريفه في factories/education
                Err(FactoryError::InitializationFailed("مصنع التعليم غير مطور بعد".to_string()))
            }
            FactoryType::Creative => {
                // سيتم تعريفه في factories/creative
                Err(FactoryError::InitializationFailed("مصنع الإبداع غير مطور بعد".to_string()))
            }
            FactoryType::Corporate => {
                // سيتم تعريفه في factories/corporate
                Err(FactoryError::InitializationFailed("مصنع المؤسسات غير مطور بعد".to_string()))
            }
            FactoryType::Technology => {
                // سيتم تعريفه في factories/technology
                Err(FactoryError::InitializationFailed("مصنع التقنية غير مطور بعد".to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_factory_type_display() {
        assert_eq!(FactoryType::Education.display_name(), "مصنع التعليم");
        assert!(FactoryType::Education.description().contains("التعليمي"));
    }
    
    #[test]
    fn test_request_priority() {
        assert_eq!(RequestPriority::Low.value(), 1);
        assert_eq!(RequestPriority::Critical.value(), 4);
    }
    
    #[test]
    fn test_factory_status() {
        assert!(FactoryStatus::Ready.is_operational());
        assert!(!FactoryStatus::Error("test".to_string()).is_operational());
    }
}
