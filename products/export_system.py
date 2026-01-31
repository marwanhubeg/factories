#!/usr/bin/env python3
"""
نظام تصدير وتنزيل المنتجات من المصانع
"""
import os
import json
import zipfile
import shutil
from datetime import datetime
from pathlib import Path

class ProductExporter:
    def __init__(self):
        self.products_dir = "products"
        self.exports_dir = "exports"
        self.setup_directories()
    
    def setup_directories(self):
        """إنشاء المجلدات المطلوبة"""
        directories = [
            self.products_dir,
            self.exports_dir,
            f"{self.products_dir}/education",
            f"{self.products_dir}/creative", 
            f"{self.products_dir}/technology",
            f"{self.products_dir}/corporate",
            f"{self.exports_dir}/education",
            f"{self.exports_dir}/creative",
            f"{self.exports_dir}/technology", 
            f"{self.exports_dir}/corporate"
        ]
        
        for directory in directories:
            Path(directory).mkdir(parents=True, exist_ok=True)
    
    def create_product(self, factory_type, product_name, content, format_type="json"):
        """إنشاء منتج جديد"""
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        product_id = f"{factory_type}_{timestamp}"
        
        product_data = {
            "id": product_id,
            "name": product_name,
            "factory_type": factory_type,
            "created_at": datetime.now().isoformat(),
            "format": format_type,
            "content": content,
            "metadata": {
                "version": "1.0",
                "author": "Marwan Hub Factories",
                "license": "MIT"
            }
        }
        
        # حفظ المنتج
        filename = f"{product_id}.{format_type}"
        filepath = f"{self.products_dir}/{factory_type}/{filename}"
        
        if format_type == "json":
            with open(filepath, 'w', encoding='utf-8') as f:
                json.dump(product_data, f, ensure_ascii=False, indent=2)
        elif format_type == "txt":
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(f"منتج: {product_name}\n")
                f.write(f"المصنع: {factory_type}\n")
                f.write(f"التاريخ: {timestamp}\n\n")
                f.write(content)
        elif format_type == "md":
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(f"# {product_name}\n\n")
                f.write(f"**المصنع:** {factory_type}\n")
                f.write(f"**تاريخ الإنتاج:** {timestamp}\n\n")
                f.write("## المحتوى\n\n")
                f.write(content)
        
        print(f"✅ تم إنشاء المنتج: {filename}")
        return product_id
    
    def export_product(self, product_id, export_format="zip"):
        """تصدير المنتج للتنزيل"""
        # البحث عن المنتج
        product_path = None
        factory_type = None
        
        for ftype in ['education', 'creative', 'technology', 'corporate']:
            for file in os.listdir(f"{self.products_dir}/{ftype}"):
                if product_id in file:
                    product_path = f"{self.products_dir}/{ftype}/{file}"
                    factory_type = ftype
                    break
            if product_path:
                break
        
        if not product_path:
            print(f"❌ المنتج {product_id} غير موجود")
            return None
        
        # إنشاء ملف التصدير
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        export_filename = f"{product_id}_export_{timestamp}"
        
        if export_format == "zip":
            # إنشاء ملف ZIP
            zip_path = f"{self.exports_dir}/{factory_type}/{export_filename}.zip"
            
            with zipfile.ZipFile(zip_path, 'w') as zipf:
                zipf.write(product_path, os.path.basename(product_path))
                
                # إضافة ملف README
                readme_content = f"""
منتج من Marwan Hub Factories
===========================
المعرف: {product_id}
المصنع: {factory_type}
تاريخ التصدير: {datetime.now().isoformat()}
                """
                zipf.writestr("README.txt", readme_content)
            
            print(f"📦 تم تصدير المنتج: {zip_path}")
            return zip_path
        
        elif export_format == "json":
            # نسخ ملف JSON مباشرة
            export_path = f"{self.exports_dir}/{factory_type}/{export_filename}.json"
            shutil.copy2(product_path, export_path)
            print(f"📄 تم تصدير المنتج: {export_path}")
            return export_path
        
        elif export_format == "html":
            # إنشاء صفحة HTML للمنتج
            with open(product_path, 'r', encoding='utf-8') as f:
                if product_path.endswith('.json'):
                    data = json.load(f)
                    content = json.dumps(data, ensure_ascii=False, indent=2)
                else:
                    content = f.read()
            
            html_content = f"""
<!DOCTYPE html>
<html lang="ar" dir="rtl">
<head>
    <meta charset="UTF-8">
    <title>{product_id} - Marwan Hub Factories</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .product {{ background: #f5f5f5; padding: 20px; border-radius: 10px; }}
        pre {{ background: white; padding: 15px; border-radius: 5px; }}
    </style>
</head>
<body>
    <h1>🏭 منتج Marwan Hub Factories</h1>
    <div class="product">
        <h2>{product_id}</h2>
        <p><strong>تم التصدير:</strong> {datetime.now().isoformat()}</p>
        <pre>{content}</pre>
    </div>
</body>
</html>
            """
            
            export_path = f"{self.exports_dir}/{factory_type}/{export_filename}.html"
            with open(export_path, 'w', encoding='utf-8') as f:
                f.write(html_content)
            
            print(f"🌐 تم تصدير المنتج: {export_path}")
            return export_path
    
    def list_products(self, factory_type=None):
        """عرض قائمة المنتجات"""
        products = []
        
        if factory_type:
            types = [factory_type]
        else:
            types = ['education', 'creative', 'technology', 'corporate']
        
        print("\n📋 المنتجات المتاحة:")
        print("="*50)
        
        for ftype in types:
            products_dir = f"{self.products_dir}/{ftype}"
            if os.path.exists(products_dir):
                files = os.listdir(products_dir)
                if files:
                    print(f"\n🏭 {ftype} ({len(files)} منتج):")
                    for file in files:
                        filepath = f"{products_dir}/{file}"
                        size = os.path.getsize(filepath)
                        products.append({
                            "id": file.replace('.json', '').replace('.txt', '').replace('.md', ''),
                            "type": ftype,
                            "file": file,
                            "size": size,
                            "path": filepath
                        })
                        print(f"  • {file} ({size:,} bytes)")
                else:
                    print(f"\n🏭 {ftype}: لا توجد منتجات")
        
        return products
    
    def download_product(self, product_id):
        """محاكاة عملية التنزيل"""
        export_path = self.export_product(product_id, "zip")
        
        if export_path and os.path.exists(export_path):
            size = os.path.getsize(export_path)
            
            print(f"\n⬇️ محاكاة تنزيل المنتج {product_id}:")
            print(f"📁 الملف: {os.path.basename(export_path)}")
            print(f"📏 الحجم: {size:,} بايت")
            print(f"📍 المسار: {export_path}")
            
            # إنشاء رابط تنزيل وهمي
            download_link = f"http://localhost:8000/exports/{os.path.basename(export_path)}"
            print(f"🔗 رابط التنزيل: {download_link}")
            
            return {
                "success": True,
                "product_id": product_id,
                "download_url": download_link,
                "file_path": export_path,
                "file_size": size,
                "timestamp": datetime.now().isoformat()
            }
        
        return {"success": False, "error": "فشل التصدير"}
    
    def batch_export(self, factory_type, export_format="zip"):
        """تصدير دفعة من المنتجات"""
        products = self.list_products(factory_type)
        
        if not products:
            print(f"❌ لا توجد منتجات في {factory_type}")
            return
        
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        batch_filename = f"{factory_type}_batch_{timestamp}.zip"
        batch_path = f"{self.exports_dir}/{batch_filename}"
        
        with zipfile.ZipFile(batch_path, 'w') as zipf:
            for product in products:
                if product['type'] == factory_type:
                    zipf.write(product['path'], f"{product['type']}/{product['file']}")
        
        size = os.path.getsize(batch_path)
        print(f"\n📦 تم تصدير دفعة من {factory_type}:")
        print(f"📁 الملف: {batch_filename}")
        print(f"📏 الحجم: {size:,} بايت")
        print(f"📊 عدد المنتجات: {len(products)}")
        print(f"📍 المسار: {batch_path}")
        
        return batch_path

# واجهة سطر الأوامر
def main():
    exporter = ProductExporter()
    
    print("🚀 نظام تصدير منتجات Marwan Hub Factories")
    print("="*50)
    
    # إنشاء بعض المنتجات التجريبية
    print("\n🔨 إنشاء منتجات تجريبية...")
    
    # منتج تعليمي
    exporter.create_product(
        "education",
        "دورة الذكاء الاصطناعي للمبتدئين",
        """# دورة الذكاء الاصطناعي

## المحتوى:
1. مقدمة في الذكاء الاصطناعي
2. تعلم الآلة
3. الشبكات العصبية
4. التطبيقات العملية

## المدة: 4 أسابيع
## المستوى: مبتدئ
""",
        "md"
    )
    
    # منتج إبداعي
    exporter.create_product(
        "creative",
        "تصميم شعار تكنولوجي",
        """{
    "design_type": "شعار",
    "style": "تكنولوجي حديث",
    "colors": ["#4361ee", "#4cc9f0", "#3a0ca3"],
    "elements": ["شريحة", "موجات", "نجوم"],
    "formats": ["SVG", "PNG", "AI"]
}""",
        "json"
    )
    
    # منتج تقني
    exporter.create_product(
        "technology",
        "مكتبة معالجة النصوص العربية",
        """// مكتبة معالجة النصوص العربية
// إصدار 1.0.0

pub struct ArabicTextProcessor {
    text: String,
}

impl ArabicTextProcessor {
    pub fn new(text: &str) -> Self {
        ArabicTextProcessor {
            text: text.to_string(),
        }
    }
    
    pub fn normalize(&self) -> String {
        // تطبيع النص العربي
        self.text.clone()
    }
    
    pub fn tokenize(&self) -> Vec<String> {
        // تقسيم النص إلى كلمات
        self.text.split_whitespace().map(|s| s.to_string()).collect()
    }
}""",
        "txt"
    )
    
    # منتج أعمال
    exporter.create_product(
        "corporate",
        "تحليل سوق التكنولوجيا 2026",
        """تقرير تحليل سوق التكنولوجيا 2026

## ملخص تنفيذي:
- نمو متوقع بنسبة 15% في قطاع الذكاء الاصطناعي
- زيادة الاستثمار في التقنيات الناشئة
- تحول رقمي متسارع في القطاعين العام والخاص

## التوصيات:
1. الاستثمار في البنية التحتية السحابية
2. تطوير مهارات الموظفين التقنية
3. تبني حلول الذكاء الاصطناعي""",
        "md"
    )
    
    # عرض المنتجات
    products = exporter.list_products()
    
    # تصدير منتج كمثال
    if products:
        first_product = products[0]
        print(f"\n📤 تصدير المنتج الأول: {first_product['id']}")
        download_info = exporter.download_product(first_product['id'])
        
        if download_info['success']:
            print(f"✅ جاهز للتنزيل: {download_info['download_url']}")
    
    # تصدير دفعة
    print(f"\n📦 تصدير دفعة من منتجات التعليم...")
    batch_path = exporter.batch_export("education")
    
    if batch_path:
        print(f"✅ دفعة جاهزة: {batch_path}")
    
    print("\n🎉 نظام التصدير جاهز للاستخدام!")

if __name__ == "__main__":
    main()
