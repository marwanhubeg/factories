#!/usr/bin/env python3
"""
خادم API مبسط للمصانع - يعمل فوراً!
"""
from http.server import HTTPServer, BaseHTTPRequestHandler
import json
import os

class FactoryAPI(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == '/api/factories':
            factories = self.get_factories()
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.send_header('Access-Control-Allow-Origin', '*')
            self.end_headers()
            self.wfile.write(json.dumps(factories).encode())
        
        elif self.path == '/api/health':
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.end_headers()
            self.wfile.write(json.dumps({
                "status": "ok",
                "version": "3.0.0",
                "factories_count": 4
            }).encode())
        
        elif self.path.startswith('/api/factory/'):
            factory_id = self.path.split('/')[-1]
            factory = self.get_factory_by_id(factory_id)
            
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.send_header('Access-Control-Allow-Origin', '*')
            self.end_headers()
            self.wfile.write(json.dumps(factory).encode())
        
        else:
            self.send_response(404)
            self.end_headers()
    
    def do_POST(self):
        if self.path == '/api/factory/start':
            content_length = int(self.headers['Content-Length'])
            post_data = self.rfile.read(content_length)
            data = json.loads(post_data)
            
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.send_header('Access-Control-Allow-Origin', '*')
            self.end_headers()
            
            response = {
                "success": True,
                "message": f"✅ تم تشغيل المصنع {data.get('id', 'unknown')}",
                "factory_id": data.get('id'),
                "status": "running"
            }
            self.wfile.write(json.dumps(response).encode())
    
    def get_factories(self):
        """الحصول على قائمة المصانع الحقيقية من المجلدات"""
        factories = []
        
        # البحث عن المصانع الحقيقية في src/factories/
        factory_types = ['education', 'creative', 'technology', 'corporate']
        
        for i, factory_type in enumerate(factory_types):
            factory_path = f"src/factories/{factory_type}"
            exists = os.path.exists(factory_path)
            
            factories.append({
                "id": f"factory_{i+1}",
                "name": self.get_factory_name(factory_type),
                "type": factory_type,
                "status": "✅ نشط" if exists else "⚠️ غير جاهز",
                "path": factory_path,
                "exists": exists,
                "files": self.count_files(factory_path) if exists else 0
            })
        
        return factories
    
    def get_factory_by_id(self, factory_id):
        """الحصول على مصنع محدد"""
        factory_types = ['education', 'creative', 'technology', 'corporate']
        index = int(factory_id.split('_')[-1]) - 1 if '_' in factory_id else 0
        
        if 0 <= index < len(factory_types):
            factory_type = factory_types[index]
            factory_path = f"src/factories/{factory_type}"
            exists = os.path.exists(factory_path)
            
            return {
                "id": factory_id,
                "name": self.get_factory_name(factory_type),
                "type": factory_type,
                "status": "✅ نشط" if exists else "⚠️ غير جاهز",
                "path": factory_path,
                "exists": exists,
                "files": self.count_files(factory_path) if exists else 0,
                "templates": self.get_templates(factory_type)
            }
        
        return {"error": "Factory not found"}
    
    def get_factory_name(self, factory_type):
        """ترجمة نوع المصنع إلى اسم عربي"""
        names = {
            'education': 'مصنع التعليم',
            'creative': 'مصنع الإبداع', 
            'technology': 'مصنع التقنية',
            'corporate': 'مصنع الشركات'
        }
        return names.get(factory_type, f"مصنع {factory_type}")
    
    def count_files(self, path):
        """عد الملفات في مجلد"""
        try:
            return len([f for f in os.listdir(path) if os.path.isfile(os.path.join(path, f))])
        except:
            return 0
    
    def get_templates(self, factory_type):
        """الحصول على القوالب المتاحة"""
        templates_path = f"templates/{factory_type}"
        if os.path.exists(templates_path):
            try:
                return os.listdir(templates_path)
            except:
                return []
        return []

def run_server(port=8081):
    server = HTTPServer(('localhost', port), FactoryAPI)
    print(f"🚀 API Server running at http://localhost:{port}")
    print(f"📡 Endpoints:")
    print(f"  • GET /api/factories")
    print(f"  • GET /api/factory/[id]")
    print(f"  • POST /api/factory/start")
    print(f"  • GET /api/health")
    server.serve_forever()

if __name__ == "__main__":
    run_server()
