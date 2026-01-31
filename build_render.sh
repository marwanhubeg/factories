#!/bin/bash
echo "🔨 بناء مشروع Render..."

# تنظيف
cargo clean 2>/dev/null || true

# إنشاء ملفات مصانع مبسطة
for factory in education creative corporate technology; do
    cat > "src/factories/${factory}/mod.rs" << FACTORY
use serde_json::Value;
use crate::core::factory::Factory;

pub struct ${factory^}Factory {
    name: String,
}

impl ${factory^}Factory {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

impl Factory for ${factory^}Factory {
    fn name(&self) -> &str { &self.name }
    fn factory_type(&self) -> &str { "${factory}" }
    fn status(&self) -> String { "active".to_string() }
    fn production_count(&self) -> u64 { 100 }
    fn quality_score(&self) -> f32 { 0.9 }
    
    fn manufacture(&self, input: Value) -> Result<Value, String> {
        Ok(serde_json::json!({
            "type": "${factory}",
            "content": "محتوى تجريبي",
            "quality": 0.85,
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }
}
FACTORY
done

# بناء المشروع
echo "🚀 جاري البناء..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ تم البناء بنجاح!"
    echo ""
    echo "📋 الخطوات التالية:"
    echo "1. رفع التغييرات إلى GitHub:"
    echo "   git add ."
    echo "   git commit -m 'إعداد للنشر على Render'"
    echo "   git push"
    echo ""
    echo "2. الذهاب إلى: https://dashboard.render.com"
    echo "3. اختيار 'New Web Service'"
    echo "4. ربط GitHub repository"
    echo "5. سيتم اكتشاف render.yaml تلقائياً"
else
    echo "❌ فشل البناء!"
fi
