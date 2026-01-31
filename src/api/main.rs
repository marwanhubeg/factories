//! نقطة دخول خادم API المستقل

use marwan_hub_factories::api::ApiServer;
use marwan_hub_factories::core::factory_manager::FactoryManager;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // إعداد التسجيل
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();
    
    // عرض شعار النظام
    print_banner();
    
    // إنشاء مدير المصانع
    let factory_manager = Arc::new(FactoryManager::new());
    
    // تهيئة المصانع الافتراضية
    factory_manager.initialize_default_factories();
    
    // خادم API
    let api_server = ApiServer::new(factory_manager, 8080);
    
    println!("🌐 API Server running on: http://localhost:8080");
    println!("📖 API Documentation: http://localhost:8080/docs");
    println!("🖥️  Dashboard: http://localhost:8080/dashboard");
    println!("⏹️  Press Ctrl+C to stop");
    
    // تشغيل الخادم
    api_server.run().await?;
    
    Ok(())
}

fn print_banner() {
    println!();
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║                                                          ║");
    println!("║                 M A R W A N   H U B                     ║");
    println!("║                 F A C T O R I E S   A P I               ║");
    println!("║                                                          ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();
    println!("🚀 API Server v{}", env!("CARGO_PKG_VERSION"));
    println!("📅 {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
    println!();
}
