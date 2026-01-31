#!/usr/bin/env python3
"""
خادم مخصص للتصدير والتنزيل
"""
from http.server import HTTPServer, BaseHTTPRequestHandler
import json
import os
import zipfile
import io
from datetime import datetime
from pathlib import Path
import urllib.parse

class ExportServer(BaseHTTPRequestHandler):
    def do_GET(self):
        # معالجة طلبات التنزيل
        if self.path.startswith('/download/'):
            self.handle_download()
        elif self.path == '/api/export/products':
            self.list_products()
        elif self.path.startswith('/api/export/batch/'):
            self.download_batch()
        elif self.path == '/api/health':
            self.health_check()
        elif self.path == '/products':
            self.serve_products_page()
        elif self.path == '/upload':
            self.serve_upload_page()
        else:
            # محاولة خدمة ملف ثابت
            self.serve_static()
    
    def do_POST(self):
        if self.path == '/api/export/create':
            self.create_product()
        else:
            self.send_error(404)
    
    def handle_download(self):
        """معالجة طلبات التنزيل"""
        # استخراج معرف المنتج من المسار
        product_id = self.path.replace('/download/', '')
        
        # البحث عن المنتج في جميع المجلدات
        found = False
        for factory_type in ['education', 'creative', 'technology', 'corporate']:
            products_dir = f"products/{factory_type}"
            if os.path.exists(products_dir):
                for filename in os.listdir(products_dir):
                    if product_id in filename:
                        filepath = os.path.join(products_dir, filename)
                        self.send_file(filepath, filename)
                        found = True
                        break
                if found:
                    break
        
        if not found:
            self.send_error(404, f"المنتج {product_id} غير موجود")
    
    def list_products(self):
        """عرض قائمة المنتجات"""
        products = []
        
        for factory_type in ['education', 'creative', 'technology', 'corporate']:
            products_dir = f"products/{factory_type}"
            if os.path.exists(products_dir):
                for filename in os.listdir(products_dir):
                    if filename.endswith(('.json', '.txt', '.md', '.html')):
                        filepath = os.path.join(products_dir, filename)
                        stat = os.stat(filepath)
                        
                        # استخراج معرف المنتج من اسم الملف
                        product_id = filename.split('.')[0]
                        
                        products.append({
                            "id": product_id,
                            "filename": filename,
                            "name": filename.replace('_', ' ').replace('.', ' ').title(),
                            "type": factory_type,
                            "size": stat.st_size,
                            "created": datetime.fromtimestamp(stat.st_ctime).isoformat(),
                            "modified": datetime.fromtimestamp(stat.st_mtime).isoformat(),
                            "download_url": f"/download/{product_id}",
                            "direct_url": f"/products/{factory_type}/{filename}"
                        })
        
        self.send_json_response({
            "success": True,
            "count": len(products),
            "products": products,
            "timestamp": datetime.now().isoformat()
        })
    
    def download_batch(self):
        """تنزيل دفعة من المنتجات"""
        try:
            factory_type = self.path.split('/')[-1]
            
            if factory_type == 'all':
                # جميع المنتجات
                products_dirs = ['education', 'creative', 'technology', 'corporate']
            else:
                products_dirs = [factory_type]
            
            # إنشاء ملف ZIP في الذاكرة
            zip_buffer = io.BytesIO()
            
            with zipfile.ZipFile(zip_buffer, 'w', zipfile.ZIP_DEFLATED) as zipf:
                added_files = 0
                
                for pdir in products_dirs:
                    products_dir = f"products/{pdir}"
                    if os.path.exists(products_dir):
                        for filename in os.listdir(products_dir):
                            if filename.endswith(('.json', '.txt', '.md', '.html')):
                                filepath = os.path.join(products_dir, filename)
                                arcname = f"{pdir}/{filename}"
                                zipf.write(filepath, arcname)
                                added_files += 1
            
            if added_files == 0:
                self.send_error(404, "لا توجد منتجات للتصدير")
                return
            
            # إرسال ملف ZIP
            zip_buffer.seek(0)
            
            self.send_response(200)
            self.send_header('Content-Type', 'application/zip')
            self.send_header('Content-Disposition', 
                           f'attachment; filename="{factory_type}_products_{datetime.now().strftime("%Y%m%d")}.zip"')
            self.send_header('Content-Length', str(zip_buffer.getbuffer().nbytes))
            self.end_headers()
            
            self.wfile.write(zip_buffer.read())
            
        except Exception as e:
            self.send_error(500, f"خطأ في إنشاء الدفعة: {str(e)}")
    
    def create_product(self):
        """إنشاء منتج جديد"""
        content_length = int(self.headers['Content-Length'])
        post_data = self.rfile.read(content_length)
        data = json.loads(post_data)
        
        factory_type = data.get('factory_type')
        product_name = data.get('name')
        content = data.get('content')
        format_type = data.get('format', 'json')
        
        if not all([factory_type, product_name, content]):
            self.send_error(400, "بيانات ناقصة")
            return
        
        # إنشاء المنتج
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        safe_name = product_name.replace(' ', '_').replace('/', '_')
        product_id = f"{factory_type}_{safe_name}_{timestamp}"
        filename = f"{product_id}.{format_type}"
        
        # إنشاء المجلد إذا لم يكن موجوداً
        products_dir = f"products/{factory_type}"
        os.makedirs(products_dir, exist_ok=True)
        
        filepath = os.path.join(products_dir, filename)
        
        try:
            if format_type == 'json':
                product_data = {
                    "id": product_id,
                    "name": product_name,
                    "factory_type": factory_type,
                    "created_at": datetime.now().isoformat(),
                    "content": content,
                    "metadata": data.get('metadata', {})
                }
                with open(filepath, 'w', encoding='utf-8') as f:
                    json.dump(product_data, f, ensure_ascii=False, indent=2)
            
            elif format_type in ['txt', 'md', 'html']:
                with open(filepath, 'w', encoding='utf-8') as f:
                    f.write(content)
            
            # إضافة ملف README
            readme_path = os.path.join(products_dir, f"{product_id}_README.txt")
            with open(readme_path, 'w', encoding='utf-8') as f:
                f.write(f"منتج: {product_name}\n")
                f.write(f"المصنع: {factory_type}\n")
                f.write(f"التاريخ: {datetime.now().isoformat()}\n")
                f.write(f"المعرف: {product_id}\n")
                f.write(f"الصيغة: {format_type}\n\n")
                f.write("--- تم إنشاء هذا المنتج بواسطة Marwan Hub Factories ---\n")
            
            self.send_json_response({
                "success": True,
                "message": f"تم إنشاء المنتج '{product_name}' بنجاح",
                "product_id": product_id,
                "filename": filename,
                "download_url": f"/download/{product_id}",
                "filepath": filepath
            })
            
        except Exception as e:
            self.send_error(500, f"خطأ في حفظ المنتج: {str(e)}")
    
    def send_file(self, filepath, filename):
        """إرسال ملف للتنزيل"""
        try:
            with open(filepath, 'rb') as f:
                content = f.read()
            
            self.send_response(200)
            self.send_header('Content-Type', self.get_mime_type(filepath))
            self.send_header('Content-Disposition', f'attachment; filename="{filename}"')
            self.send_header('Content-Length', str(len(content)))
            self.end_headers()
            
            self.wfile.write(content)
            
        except Exception as e:
            self.send_error(500, f"خطأ في قراءة الملف: {str(e)}")
    
    def serve_static(self):
        """خدمة الملفات الثابتة"""
        # محاولة خدمة ملف من المسار المطلوب
        path = self.path.lstrip('/')
        
        if path == '':
            path = 'index.html'
        
        # قائمة الملفات المسموح بها
        allowed_files = [
            'index.html', 'dashboard_live.html', 'download_products.html', 
            'upload_product.html', 'favicon.ico'
        ]
        
        if path in allowed_files and os.path.exists(path):
            self.serve_file(path)
        elif path.startswith('products/') and os.path.exists(path):
            # السماح بالوصول للمنتجات مباشرة
            self.serve_file(path)
        else:
            self.send_error(404)
    
    def serve_file(self, filepath):
        """خدمة ملف عادي"""
        try:
            with open(filepath, 'rb') as f:
                content = f.read()
            
            self.send_response(200)
            self.send_header('Content-Type', self.get_mime_type(filepath))
            self.send_header('Content-Length', str(len(content)))
            self.end_headers()
            
            self.wfile.write(content)
            
        except Exception as e:
            self.send_error(500, f"خطأ في قراءة الملف: {str(e)}")
    
    def serve_products_page(self):
        """خدمة صفحة المنتجات"""
        if os.path.exists('download_products.html'):
            self.serve_file('download_products.html')
        else:
            self.send_error(404, "صفحة المنتجات غير موجودة")
    
    def serve_upload_page(self):
        """خدمة صفحة الرفع"""
        if os.path.exists('upload_product.html'):
            self.serve_file('upload_product.html')
        else:
            self.send_error(404, "صفحة الرفع غير موجودة")
    
    def health_check(self):
        """فحص صحة الخادم"""
        self.send_json_response({
            "status": "ok",
            "version": "3.2.0",
            "service": "Export Server",
            "timestamp": datetime.now().isoformat(),
            "products_count": self.count_products()
        })
    
    def send_json_response(self, data):
        """إرسال رد JSON"""
        self.send_response(200)
        self.send_header('Content-Type', 'application/json; charset=utf-8')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()
        
        json_str = json.dumps(data, ensure_ascii=False, indent=2)
        self.wfile.write(json_str.encode('utf-8'))
    
    def get_mime_type(self, filename):
        """الحصول على نوع MIME للملف"""
        mime_types = {
            '.html': 'text/html',
            '.htm': 'text/html',
            '.json': 'application/json',
            '.txt': 'text/plain',
            '.md': 'text/markdown',
            '.zip': 'application/zip',
            '.ico': 'image/x-icon',
            '.png': 'image/png',
            '.jpg': 'image/jpeg',
            '.jpeg': 'image/jpeg',
            '.css': 'text/css',
            '.js': 'application/javascript'
        }
        
        ext = os.path.splitext(filename)[1].lower()
        return mime_types.get(ext, 'application/octet-stream')
    
    def count_products(self):
        """عد المنتجات"""
        count = 0
        for factory_type in ['education', 'creative', 'technology', 'corporate']:
            products_dir = f"products/{factory_type}"
            if os.path.exists(products_dir):
                count += len([f for f in os.listdir(products_dir) 
                            if f.endswith(('.json', '.txt', '.md', '.html'))])
        return count

def run_server(port=9000):
    """تشغيل الخادم"""
    os.chdir(os.path.dirname(os.path.abspath(__file__)))
    
    # إنشاء مجلدات المنتجات
    for factory_type in ['education', 'creative', 'technology', 'corporate']:
        os.makedirs(f"products/{factory_type}", exist_ok=True)
    
    server = HTTPServer(('0.0.0.0', port), ExportServer)
    
    print("\n" + "="*60)
    print("🚀 خادم تصدير وتنزيل المنتجات - Marwan Hub Factories")
    print("="*60)
    print(f"🌐 يعمل على: http://localhost:{port}")
    print(f"📁 المجلد الحالي: {os.getcwd()}")
    print("\n📋 نقاط الوصول:")
    print(f"  • 📥 مركز التنزيل:    http://localhost:{port}/products")
    print(f"  • 📤 رفع منتج جديد:   http://localhost:{port}/upload")
    print(f"  • 📋 قائمة المنتجات:  http://localhost:{port}/api/export/products")
    print(f"  • 🩺 فحص الصحة:       http://localhost:{port}/api/health")
    print(f"  • 🏠 الصفحة الرئيسية: http://localhost:{port}/")
    print("\n⚡ أمثلة تنزيل:")
    print(f"  • تنزيل منتج:        http://localhost:{port}/download/product_id")
    print(f"  • دفعة التعليم:      http://localhost:{port}/api/export/batch/education")
    print(f"  • جميع المنتجات:     http://localhost:{port}/api/export/batch/all")
    print("="*60 + "\n")
    
    server.serve_forever()

if __name__ == "__main__":
    run_server()
