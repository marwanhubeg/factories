#!/usr/bin/env python3
"""
Marwan Hub Factories v3.0.0 - Unified Server
خادم موحد يجمع كل خدمات النظام في خادم واحد
"""

import os
import sys
import json
import time
import signal
import logging
import threading
import subprocess
from datetime import datetime
from pathlib import Path
from http.server import HTTPServer, SimpleHTTPRequestHandler
from urllib.parse import urlparse, parse_qs
import mimetypes

# إعدادات المسارات
BASE_DIR = Path(".")
PRODUCTS_DIR = BASE_DIR / "products"
EXPORTS_DIR = BASE_DIR / "exports"
TEMPLATES_DIR = BASE_DIR / "templates"
DOWNLOADS_DIR = BASE_DIR / "downloads"
DOWNLOADS_DIR.mkdir(exist_ok=True)

# إعداد التسجيل
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler('unified_server.log'),
        logging.StreamHandler()
    ]
)
logger = logging.getLogger("UnifiedServer")

class UnifiedRequestHandler(SimpleHTTPRequestHandler):
    """معالج طلبات موحد لكل خدمات النظام"""
    
    def do_GET(self):
        """معالجة طلبات GET"""
        try:
            parsed_path = urlparse(self.path)
            path = parsed_path.path
            
            logger.info(f"GET request: {path}")
            
            # API endpoints
            if path.startswith('/api/'):
                self.handle_api(path)
            # Download endpoints
            elif path.startswith('/download/'):
                self.handle_download(path)
            # Static files
            else:
                self.handle_static_file(path)
                
        except Exception as e:
            logger.error(f"Error handling GET: {e}")
            self.send_error(500, str(e))
    
    def do_POST(self):
        """معالجة طلبات POST"""
        try:
            parsed_path = urlparse(self.path)
            path = parsed_path.path
            
            logger.info(f"POST request: {path}")
            
            # قراءة محتوى الطلب
            content_length = int(self.headers['Content-Length'])
            post_data = self.rfile.read(content_length)
            
            if path == '/api/factories/start':
                self.handle_factory_start(post_data)
            elif path == '/api/export':
                self.handle_export(post_data)
            elif path == '/api/upload':
                self.handle_upload(post_data)
            else:
                self.send_error(404, "Endpoint not found")
                
        except Exception as e:
            logger.error(f"Error handling POST: {e}")
            self.send_error(500, str(e))
    
    def handle_api(self, path):
        """معالجة طلبات API"""
        if path == '/api/factories':
            self.get_factories_status()
        elif path == '/api/products':
            self.get_products_list()
        elif path == '/api/exports':
            self.get_exports_list()
        else:
            self.send_error(404, "API endpoint not found")
    
    def handle_download(self, path):
        """معالجة طلبات التنزيل"""
        filename = path.replace('/download/', '')
        filepath = EXPORTS_DIR / filename
        
        if not filepath.exists():
            self.send_error(404, "File not found")
            return
            
        try:
            with open(filepath, 'rb') as f:
                file_data = f.read()
            
            self.send_response(200)
            self.send_header('Content-Type', 'application/octet-stream')
            self.send_header('Content-Disposition', f'attachment; filename="{filename}"')
            self.send_header('Content-Length', str(len(file_data)))
            self.end_headers()
            self.wfile.write(file_data)
            
        except Exception as e:
            logger.error(f"Download error: {e}")
            self.send_error(500, str(e))
    
    def handle_static_file(self, path):
        """معالجة الملفات الثابتة"""
        if path == '/':
            path = '/index.html'
        
        filepath = BASE_DIR / path.lstrip('/')
        
        if not filepath.exists():
            self.send_error(404, "File not found")
            return
            
        try:
            with open(filepath, 'rb') as f:
                file_data = f.read()
            
            # تحديد نوع الملف
            mime_type, _ = mimetypes.guess_type(str(filepath))
            if not mime_type:
                mime_type = 'application/octet-stream'
            
            self.send_response(200)
            self.send_header('Content-Type', mime_type)
            self.send_header('Content-Length', str(len(file_data)))
            self.end_headers()
            self.wfile.write(file_data)
            
        except Exception as e:
            logger.error(f"Static file error: {e}")
            self.send_error(500, str(e))
    
    def get_factories_status(self):
        """الحصول على حالة المصانع"""
        status_file = BASE_DIR / "factories_real_status.json"
        
        if status_file.exists():
            with open(status_file, 'r', encoding='utf-8') as f:
                status_data = json.load(f)
        else:
            # بيانات افتراضية
            status_data = {
                "education": {"status": "ready", "products": 3},
                "creative": {"status": "ready", "products": 3},
                "corporate": {"status": "ready", "products": 3},
                "technology": {"status": "ready", "products": 3}
            }
        
        self.send_json_response(status_data)
    
    def get_products_list(self):
        """الحصول على قائمة المنتجات"""
        products = []
        
        for factory_type in ['education', 'creative', 'corporate', 'technology']:
            factory_dir = PRODUCTS_DIR / factory_type
            if factory_dir.exists():
                for file in factory_dir.iterdir():
                    if file.is_file() and file.name != 'README.txt':
                        products.append({
                            "factory": factory_type,
                            "name": file.name,
                            "path": f"/products/{factory_type}/{file.name}",
                            "size": file.stat().st_size,
                            "created": datetime.fromtimestamp(file.stat().st_ctime).isoformat()
                        })
        
        self.send_json_response({"products": products})
    
    def get_exports_list(self):
        """الحصول على قائمة المنتجات المصدرة"""
        exports = []
        
        if EXPORTS_DIR.exists():
            for file in EXPORTS_DIR.iterdir():
                if file.is_file() and file.suffix in ['.zip', '.tar']:
                    exports.append({
                        "name": file.name,
                        "path": f"/exports/{file.name}",
                        "size": file.stat().st_size,
                        "created": datetime.fromtimestamp(file.stat().st_ctime).isoformat(),
                        "download_url": f"/download/{file.name}"
                    })
        
        self.send_json_response({"exports": exports})
    
    def handle_factory_start(self, post_data):
        """بدء تشغيل مصنع"""
        try:
            data = json.loads(post_data)
            factory_type = data.get('factory_type')
            
            # محاكاة بدء المصنع
            result = {
                "success": True,
                "message": f"Factory {factory_type} started successfully",
                "factory": factory_type,
                "started_at": datetime.now().isoformat(),
                "job_id": f"job_{factory_type}_{int(time.time())}"
            }
            
            self.send_json_response(result)
            
        except Exception as e:
            self.send_error(500, f"Factory start error: {e}")
    
    def handle_export(self, post_data):
        """تصدير المنتجات"""
        try:
            data = json.loads(post_data)
            factory_type = data.get('factory_type')
            
            # محاكاة عملية التصدير
            export_file = EXPORTS_DIR / f"{factory_type}_export_{int(time.time())}.zip"
            export_file.touch()  # إنشاء ملف مؤقت
            
            result = {
                "success": True,
                "message": f"Export completed for {factory_type}",
                "export_file": export_file.name,
                "download_url": f"/download/{export_file.name}",
                "size": export_file.stat().st_size
            }
            
            self.send_json_response(result)
            
        except Exception as e:
            self.send_error(500, f"Export error: {e}")
    
    def handle_upload(self, post_data):
        """رفع منتج"""
        try:
            # حفظ الملف المرفوع
            filename = f"uploaded_{int(time.time())}.dat"
            filepath = DOWNLOADS_DIR / filename
            
            with open(filepath, 'wb') as f:
                f.write(post_data)
            
            result = {
                "success": True,
                "message": "File uploaded successfully",
                "filename": filename,
                "download_url": f"/downloads/{filename}",
                "size": len(post_data)
            }
            
            self.send_json_response(result)
            
        except Exception as e:
            self.send_error(500, f"Upload error: {e}")
    
    def send_json_response(self, data):
        """إرسال رد JSON"""
        response = json.dumps(data, ensure_ascii=False).encode('utf-8')
        
        self.send_response(200)
        self.send_header('Content-Type', 'application/json; charset=utf-8')
        self.send_header('Content-Length', str(len(response)))
        self.end_headers()
        self.wfile.write(response)
    
    def log_message(self, format, *args):
        """تسجيل الرسائل بشكل منظم"""
        logger.info("%s - %s" % (self.address_string(), format % args))

def signal_handler(signum, frame):
    """معالج إشارات الإغلاق"""
    logger.info("Shutdown signal received. Stopping unified server...")
    sys.exit(0)

def main():
    """الدالة الرئيسية"""
    logger.info("=" * 60)
    logger.info("🚀 Marwan Hub Factories v3.0.0 - Unified Server")
    logger.info("=" * 60)
    
    # معالجة إشارات الإغلاق
    signal.signal(signal.SIGINT, signal_handler)
    signal.signal(signal.SIGTERM, signal_handler)
    
    # إعداد المنفذ
    PORT = 8000
    if len(sys.argv) > 1:
        try:
            PORT = int(sys.argv[1])
        except ValueError:
            logger.warning(f"Invalid port: {sys.argv[1]}, using default: {PORT}")
    
    # تشغيل الخادم الموحد
    server_address = ('', PORT)
    httpd = HTTPServer(server_address, UnifiedRequestHandler)
    
    logger.info(f"🌐 Unified server running on http://localhost:{PORT}")
    logger.info(f"📁 Products directory: {PRODUCTS_DIR.absolute()}")
    logger.info(f"📦 Exports directory: {EXPORTS_DIR.absolute()}")
    logger.info("=" * 60)
    logger.info("Available endpoints:")
    logger.info("  · /                    - الواجهة الرئيسية")
    logger.info("  · /dashboard_live.html - لوحة التحكم المباشرة")
    logger.info("  · /admin_dashboard.html - لوحة الإدارة المتقدمة")
    logger.info("  · /api/factories       - حالة المصانع")
    logger.info("  · /api/products        - قائمة المنتجات")
    logger.info("  · /api/exports         - قائمة المنتجات المصدرة")
    logger.info("  · /download/{file}     - تنزيل الملفات")
    logger.info("=" * 60)
    
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        logger.info("Server stopped by user")
    except Exception as e:
        logger.error(f"Server error: {e}")

if __name__ == '__main__':
    main()
