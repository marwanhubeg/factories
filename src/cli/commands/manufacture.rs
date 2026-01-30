use std::sync::Arc;
use crate::core::factory_manager::FactoryManager;

/// تنفيذ أمر التصنيع
pub async fn execute(
    factory_manager: Arc<FactoryManager>,
    factory_type: String,
    input: String,
    params: Option<String>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🏭 بدء عملية التصنيع:");
    println!("   المصنع: {}", factory_type);
    println!("   المدخلات: {}", input);
    
    // تحويل المدخلات إلى JSON
    let input_json = match serde_json::from_str(&input) {
        Ok(json) => json,
        Err(_) => {
            // إذا لم يكن JSON، استخدمه كنص عادي
            serde_json::json!({ "content": input })
        }
    };
    
    let params_json = params
        .map(|p| serde_json::from_str(&p).unwrap_or_default())
        .unwrap_or_default();
    
    // تنفيذ التصنيع
    match factory_manager.manufacture(&factory_type, input_json, params_json) {
        Ok(product) => {
            println!("✅ تم التصنيع بنجاح!");
            println!("📦 المنتج الناتج:");
            println!("{}", serde_json::to_string_pretty(&product).unwrap());
            
            // تحليل الجودة
            let quality_score = product.get("quality_score")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.85) as f32;
            
            println!("⭐ جودة المنتج: {:.1}%", quality_score * 100.0);
            
            if quality_score > 0.9 {
                println!("🎉 منتج ممتاز!");
            } else if quality_score > 0.7 {
                println!("👍 منتج جيد");
            } else {
                println!("⚠️  يحتاج المنتج إلى تحسين");
            }
        }
        Err(e) => {
            println!("❌ فشل في التصنيع: {}", e);
        }
    }
    
    Ok(())
}
