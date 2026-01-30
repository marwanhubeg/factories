//! مصانع مروان هوب الذكية
//! تحتوي على المصانع الأربعة المتخصصة

pub mod education;
pub mod creative;
pub mod corporate;
pub mod technology;

use crate::core::factory::{Factory, FactoryType, FactoryStatus, FactoryCapability};
use crate::core::quality::QualityGate;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// سجل المصانع
pub struct FactoriesRegistry {
    factories: HashMap<FactoryType, Box<dyn Factory>>,
}

impl FactoriesRegistry {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }
    
    pub fn register_factory(&mut self, factory: Box<dyn Factory>) {
        let factory_type = factory.get_type();
        self.factories.insert(factory_type, factory);
    }
    
    pub fn get_factory(&self, factory_type: FactoryType) -> Option<&dyn Factory> {
        self.factories.get(&factory_type).map(|f| f.as_ref())
    }
    
    pub fn list_factories(&self) -> Vec<FactoryInfo> {
        self.factories
            .values()
            .map(|factory| FactoryInfo {
                name: factory.get_name(),
                factory_type: factory.get_type(),
                status: factory.get_status(),
                version: factory.get_version(),
            })
            .collect()
    }
    
    pub fn initialize_all(&mut self) -> Result<(), String> {
        log::info!("تهيئة جميع المصانع...");
        
        // تسجيل المصانع الأربعة
        self.register_factory(Box::new(education::EducationFactory::new()));
        self.register_factory(Box::new(creative::CreativeFactory::new()));
        self.register_factory(Box::new(corporate::CorporateFactory::new()));
        self.register_factory(Box::new(technology::TechnologyFactory::new()));
        
        // تهيئة كل مصنع
        for (factory_type, factory) in &self.factories {
            log::info!("تهيئة مصنع: {:?}", factory_type);
            if let Err(e) = factory.initialize() {
                return Err(format!("فشل تهيئة المصنع {:?}: {}", factory_type, e));
            }
        }
        
        log::info!("✅ تم تهيئة جميع المصانع بنجاح");
        Ok(())
    }
    
    pub fn shutdown_all(&self) -> Result<(), String> {
        log::info!("إيقاف جميع المصانع...");
        
        for (factory_type, factory) in &self.factories {
            log::info!("إيقاف مصنع: {:?}", factory_type);
            if let Err(e) = factory.shutdown() {
                log::error!("فشل إيقاف المصنع {:?}: {}", factory_type, e);
            }
        }
        
        log::info!("✅ تم إيقاف جميع المصانع");
        Ok(())
    }
}

/// معلومات المصنع
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactoryInfo {
    pub name: String,
    pub factory_type: FactoryType,
    pub status: FactoryStatus,
    pub version: String,
}

/// تهيئة نظام المصانع
pub fn initialize_factories() -> Result<FactoriesRegistry, String> {
    log::info!("🚀 بدء تهيئة نظام المصانع...");
    
    let mut registry = FactoriesRegistry::new();
    registry.initialize_all()?;
    
    log::info!("🎉 تم تهيئة نظام المصانع بنجاح");
    Ok(registry)
}

/// المصنع الأساسي المشترك
pub struct BaseFactory {
    name: String,
    factory_type: FactoryType,
    version: String,
    status: FactoryStatus,
    capabilities: Vec<FactoryCapability>,
}

impl BaseFactory {
    pub fn new(name: String, factory_type: FactoryType, version: String) -> Self {
        Self {
            name,
            factory_type,
            version,
            status: FactoryStatus::Initializing,
            capabilities: Vec::new(),
        }
    }
    
    pub fn add_capability(&mut self, capability: FactoryCapability) {
        self.capabilities.push(capability);
    }
    
    pub fn set_status(&mut self, status: FactoryStatus) {
        self.status = status;
    }
    
    pub fn get_base_capabilities(&self) -> &[FactoryCapability] {
        &self.capabilities
    }
}

/// تنفيذ مشترك للواجهة الأساسية
impl Factory for BaseFactory {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    
    fn get_type(&self) -> FactoryType {
        self.factory_type.clone()
    }
    
    fn get_version(&self) -> String {
        self.version.clone()
    }
    
    fn get_status(&self) -> FactoryStatus {
        self.status.clone()
    }
    
    fn get_capabilities(&self) -> Vec<FactoryCapability> {
        self.capabilities.clone()
    }
    
    fn process_request(&self, _request: crate::core::factory::FactoryRequest) 
        -> Result<crate::core::factory::FactoryResponse, crate::core::factory::FactoryError> {
        Err(crate::core::factory::FactoryError::ProcessingFailed(
            "لم يتم تنفيذ هذه الوظيفة في المصنع الأساسي".to_string()
        ))
    }
    
    fn validate_output(&self, _output: &crate::core::factory::FactoryOutput) -> Vec<QualityGate> {
        Vec::new()
    }
    
    fn get_metrics(&self) -> crate::core::factory::FactoryMetrics {
        crate::core::factory::FactoryMetrics {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            avg_processing_time_ms: 0.0,
            current_queue_size: 0,
            memory_usage_mb: 0.0,
            last_reset: chrono::Utc::now(),
        }
    }
    
    fn reset(&self) -> Result<(), crate::core::factory::FactoryError> {
        Ok(())
    }
}

/// حالة نظام المصانع
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactoriesSystemState {
    pub total_factories: usize,
    pub active_factories: usize,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub average_quality_score: f64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// الحصول على حالة نظام المصانع
pub fn get_factories_system_state(registry: &FactoriesRegistry) -> FactoriesSystemState {
    let factories = registry.list_factories();
    let total_factories = factories.len();
    let active_factories = factories.iter()
        .filter(|f| f.status.is_operational())
        .count();
    
    FactoriesSystemState {
        total_factories,
        active_factories,
        total_requests: 0, // سيتم تحديثه من قاعدة البيانات
        successful_requests: 0,
        average_quality_score: 0.95, // قيمة افتراضية
        last_updated: chrono::Utc::now(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_factories_registry() {
        let registry = FactoriesRegistry::new();
        let factories = registry.list_factories();
        
        assert_eq!(factories.len(), 0); // فارغ عند الإنشاء
    }
    
    #[test]
    fn test_base_factory() {
        let factory = BaseFactory::new(
            "مصنع تجريبي".to_string(),
            FactoryType::Education,
            "1.0.0".to_string()
        );
        
        assert_eq!(factory.get_name(), "مصنع تجريبي");
        assert!(matches!(factory.get_type(), FactoryType::Education));
        assert_eq!(factory.get_version(), "1.0.0");
    }
}
