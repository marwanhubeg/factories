#!/usr/bin/env python3
import os
import json
from datetime import datetime

# إنشاء مجلدات المنتجات
for factory_type in ['education', 'creative', 'technology', 'corporate']:
    os.makedirs(f"products/{factory_type}", exist_ok=True)

# منتج تعليمي
education_product = {
    "id": "education_course_ai_20260131",
    "name": "دورة الذكاء الاصطناعي",
    "content": """# دورة الذكاء الاصطناعي

## الوصف
دورة متكاملة للذكاء الاصطناعي للمبتدئين

## المحتويات:
1. مقدمة في الذكاء الاصطناعي
2. تعلم الآلة الأساسي
3. الشبكات العصبية
4. تطبيقات عملية

## المدة: 4 أسابيع
## المستوى: مبتدئ""",
    "format": "md"
}

# منتج إبداعي
creative_product = {
    "id": "creative_design_logo_20260131",
    "name": "تصميم شعار تكنولوجي",
    "content": json.dumps({
        "design": {
            "type": "شعار",
            "style": "تكنولوجي حديث",
            "colors": ["#4361ee", "#4cc9f0", "#3a0ca3"],
            "elements": ["شريحة", "موجات", "نجوم"]
        },
        "formats": ["SVG", "PNG", "AI"],
        "usage": "للشركات التقنية الناشئة"
    }, ensure_ascii=False, indent=2),
    "format": "json"
}

# منتج تقني
technology_product = {
    "id": "technology_library_text_20260131",
    "name": "مكتبة معالجة النصوص العربية",
    "content": """// مكتبة معالجة النصوص العربية - Rust
// إصدار 1.0.0

pub struct ArabicText {
    text: String,
}

impl ArabicText {
    pub fn new(text: &str) -> Self {
        ArabicText {
            text: text.to_string(),
        }
    }
    
    pub fn normalize(&self) -> String {
        // تطبيع النص العربي
        self.text
            .replace('أ', 'ا')
            .replace('إ', 'ا')
            .replace('آ', 'ا')
            .replace('ة', 'ه')
    }
    
    pub fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }
}""",
    "format": "txt"
}

# منتج أعمال
corporate_product = {
    "id": "corporate_report_20260131",
    "name": "تقرير سوق التكنولوجيا 2026",
    "content": """# تقرير سوق التكنولوجيا 2026

## الملخص التنفيذي
تحليل شامل لسوق التكنولوجيا مع توقعات 2026

## النقاط الرئيسية:
1. نمو متوقع بنسبة 15% في الذكاء الاصطناعي
2. زيادة الاستثمار في التقنيات الناشئة
3. تحول رقمي متسارع

## التوصيات:
- الاستثمار في البنية التحتية السحابية
- تطوير المهارات الرقمية
- تبني حلول الذكاء الاصطناعي""",
    "format": "md"
}

# حفظ المنتجات
products = [
    ("education", education_product),
    ("creative", creative_product),
    ("technology", technology_product),
    ("corporate", corporate_product)
]

print("📦 إنشاء منتجات تجريبية للاختبار...")

for factory_type, product in products:
    filename = f"{product['id']}.{product['format']}"
    filepath = f"products/{factory_type}/{filename}"
    
    with open(filepath, 'w', encoding='utf-8') as f:
        if product['format'] == 'json':
            json.dump(json.loads(product['content']), f, ensure_ascii=False, indent=2)
        else:
            f.write(product['content'])
    
    print(f"✅ {factory_type}: {filename}")

# إنشاء ملفات README
for factory_type in ['education', 'creative', 'technology', 'corporate']:
    readme_path = f"products/{factory_type}/README.txt"
    with open(readme_path, 'w', encoding='utf-8') as f:
        f.write(f"منتجات {factory_type}\n")
        f.write(f"تاريخ الإنشاء: {datetime.now().isoformat()}\n")
        f.write("--- Marwan Hub Factories ---\n")

print("\n🎉 تم إنشاء 4 منتجات تجريبية بنجاح!")
print("📁 المجلدات:")
for factory_type in ['education', 'creative', 'technology', 'corporate']:
    count = len([f for f in os.listdir(f"products/{factory_type}") 
                if not f.endswith('README.txt')])
    print(f"  • products/{factory_type}: {count} منتج")
