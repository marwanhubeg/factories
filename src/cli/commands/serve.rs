use std::sync::Arc;
use crate::core::factory_manager::FactoryManager;
use crate::api::ApiServer;

/// تنفيذ أمر تشغيل الخادم
pub async fn execute(
    factory_manager: Arc<FactoryManager>,
    port: u16,
    host: Option<String>
) -> Result<(), Box<dyn std::error::Error>> {
    let host_str = host.unwrap_or_else(|| "0.0.0.0".to_string());
    
    println!("🚀 بدء تشغيل Marwan Hub Factories v3.0.0");
    println!("📡 العنوان: {}:{}", host_str, port);
    println!("📊 المصانع النشطة: {}", factory_manager.list_factories().len());
    
    // إنشاء وتهيئة المصانع
    factory_manager.initialize_default_factories();
    
    // تشغيل خادم API
    let api_server = ApiServer::new(factory_manager, port);
    
    println!("✅ النظام جاهز للاستخدام");
    println!("🔗 API Endpoint: http://{}:{}/api/v1", host_str, port);
    println!("📚 التوثيق: http://{}:{}/docs", host_str, port);
    println!("⏹️  اضغط Ctrl+C لإيقاف الخادم");
    
    api_server.run().await?;
    
    Ok(())
}
