//! المكتبة الرئيسية لنظام Marwan Hub Factories
//! الإصدار: 3.0.0

pub mod core;
pub mod factories;
pub mod api;
pub mod cli;
pub mod mhos;

use serde::Serialize;

/// معلومات النظام
#[derive(Serialize)]
pub struct SystemInfo {
    pub version: String,
    pub name: String,
    pub description: String,
    pub rust_version: String,
    pub build_time: String,
}

impl Default for SystemInfo {
    fn default() -> Self {
        SystemInfo {
            version: "3.0.0".to_string(),
            name: "Marwan Hub Factories".to_string(),
            description: "نظام المصانع الذكية للتعليم والإبداع والتقنية".to_string(),
            rust_version: std::env::var("RUSTC_VERSION")
                .unwrap_or_else(|_| "unknown".to_string()),
            build_time: std::env::var("BUILD_TIME")
                .unwrap_or_else(|_| chrono::Utc::now().to_rfc3339()),
        }
    }
}

/// الحصول على معلومات النظام
pub fn get_system_info() -> SystemInfo {
    SystemInfo::default()
}

/// تهيئة النظام
pub fn initialize() -> Result<(), String> {
    println!("🚀 تهيئة نظام المصانع...");
    println!("📦 الإصدار: 3.0.0");
    println!("✅ التهيئة مكتملة!");
    Ok(())
}
