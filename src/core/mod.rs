//! الوحدة الأساسية للنظام

pub mod factory;
pub mod quality;
pub mod factory_manager;
pub mod config;
pub mod errors;

// إعادة التصدير للاستخدام السهل
pub use factory::Factory;
pub use quality::QualityGate;
pub use factory_manager::FactoryManager;
pub use config::Config;
pub use errors::{FactoryError, FactoryResult};
