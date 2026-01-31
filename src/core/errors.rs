use std::fmt;

/// أخطاء نظام المصانع
#[derive(Debug)]
pub enum FactoryError {
    /// خطأ في التكوين
    ConfigError(String),
    
    /// خطأ في قاعدة البيانات
    DatabaseError(String),
    
    /// خطأ في المصنع
    FactoryError(String),
    
    /// خطأ في الجودة
    QualityError(String),
    
    /// خطأ في التصنيع
    ManufacturingError(String),
    
    /// خطأ في الأمان
    SecurityError(String),
    
    /// خطأ في الوصول
    AccessError(String),
    
    /// خطأ غير معروف
    UnknownError(String),
    
    /// خطأ في الإدخال/الإخراج
    IoError(std::io::Error),
}

impl fmt::Display for FactoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FactoryError::ConfigError(msg) => write!(f, "خطأ في التكوين: {}", msg),
            FactoryError::DatabaseError(msg) => write!(f, "خطأ في قاعدة البيانات: {}", msg),
            FactoryError::FactoryError(msg) => write!(f, "خطأ في المصنع: {}", msg),
            FactoryError::QualityError(msg) => write!(f, "خطأ في الجودة: {}", msg),
            FactoryError::ManufacturingError(msg) => write!(f, "خطأ في التصنيع: {}", msg),
            FactoryError::SecurityError(msg) => write!(f, "خطأ في الأمان: {}", msg),
            FactoryError::AccessError(msg) => write!(f, "خطأ في الوصول: {}", msg),
            FactoryError::UnknownError(msg) => write!(f, "خطأ غير معروف: {}", msg),
            FactoryError::IoError(err) => write!(f, "خطأ في الإدخال/الإخراج: {}", err),
        }
    }
}

impl std::error::Error for FactoryError {}

impl From<std::io::Error> for FactoryError {
    fn from(err: std::io::Error) -> Self {
        FactoryError::IoError(err)
    }
}

/// نتيجة العملية
pub type FactoryResult<T> = Result<T, FactoryError>;
