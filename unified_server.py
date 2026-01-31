#!/usr/bin/env python3
"""
خادم موحد يخدم كل شيء: HTML + API
"""
from http.server import HTTPServer, SimpleHTTPRequestHandler
import json
import os
import urllib.parse

class UnifiedServer(SimpleHTTPRequestHandler):
    def do_GET(self):
        # معالجة API requests
        if self.path.startswith('/api/'):
            self.handle_api()
            return
        
        # معالجة ملفات عادية
        super().do_GET()
    
    def handle_api(self):
        if self.path == '/api/factories' or self.path == '/api/factories/':
            self.send_json_response(self.get_factories())
        
        elif self.path == '/api/health' or self.path == '/api/health/':
            self.send_json_response({
                "status": "ok",
                "version": "3.0.0",
                "message": "Marwan Hub Factories API is running"
            })
        
        elif self.path.startswith('/api/factory/'):
            factory_id = self.path.split('/')[-1]
            self.send_json_response(self.get_factory(factory_id))
        
        else:
            self.send_error(404, "API endpoint not found")
    
    def do_POST(self):
        if self.path == '/api/factory/start':
            content_length = int(self.headers['Content-Length'])
            post_data = self.rfile.read(content_length)
            data = json.loads(post_data)
            
            response = {
                "success": True,
                "message": f"✅ تم تشغيل المصنع {data.get('id', 'unknown')}",
                "factory_id": data.get('id'),
                "status": "running",
                "timestamp": "2026-01-31T03:40:00Z"
            }
            self.send_json_response(response)
        
        else:
            self.send_error(404)
    
    def send_json_response(self, data):
        self.send_response(200)
        self.send_header('Content-Type', 'application/json; charset=utf-8')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()
        self.wfile.write(json.dumps(data, ensure_ascii=False).encode('utf-8'))
    
    def get_factories(self):
        factories = []
        factory_types = ['education', 'creative', 'technology', 'corporate']
        
        for i, ftype in enumerate(factory_types):
            path = f"src/factories/{ftype}"
            exists = os.path.isdir(path)
            
            factories.append({
                "id": f"factory_{i+1}",
                "name": self.get_arabic_name(ftype),
                "type": ftype,
                "status": "✅ نشط" if exists else "⚠️ قيد التطوير",
                "path": path,
                "exists": exists,
                "files": self.count_files(path),
                "icon": self.get_icon(ftype)
            })
        
        return factories
    
    def get_factory(self, factory_id):
        factory_types = ['education', 'creative', 'technology', 'corporate']
        
        try:
            idx = int(factory_id.split('_')[-1]) - 1
            if 0 <= idx < len(factory_types):
                ftype = factory_types[idx]
                path = f"src/factories/{ftype}"
                exists = os.path.isdir(path)
                
                return {
                    "id": factory_id,
                    "name": self.get_arabic_name(ftype),
                    "type": ftype,
                    "status": "✅ نشط" if exists else "⚠️ قيد التطوير",
                    "path": path,
                    "exists": exists,
                    "files": self.count_files(path),
                    "templates": self.get_templates(ftype),
                    "description": self.get_description(ftype)
                }
        except:
            pass
        
        return {"error": "المصنع غير موجود"}
    
    def get_arabic_name(self, ftype):
        names = {
            'education': 'مصنع التعليم',
            'creative': 'مصنع الإبداع',
            'technology': 'مصنع التقنية',
            'corporate': 'مصنع الشركات'
        }
        return names.get(ftype, ftype)
    
    def get_icon(self, ftype):
        icons = {
            'education': '🎓',
            'creative': '🎨',
            'technology': '💻',
            'corporate': '📊'
        }
        return icons.get(ftype, '🏭')
    
    def get_description(self, ftype):
        desc = {
            'education': 'إنتاج محتوى تعليمي، دورات، ومناهج تعليمية ذكية',
            'creative': 'تصميمات إبداعية، محتوى بصري، وعناصر إعلامية',
            'technology': 'تطوير برمجيات، واجهات برمجة، وحلول تقنية',
            'corporate': 'حلول أعمال، تحليلات، واستشارات مؤسسية'
        }
        return desc.get(ftype, 'مصنع إنتاجي')
    
    def count_files(self, path):
        if not os.path.exists(path):
            return 0
        try:
            return len([f for f in os.listdir(path) if os.path.isfile(os.path.join(path, f))])
        except:
            return 0
    
    def get_templates(self, ftype):
        path = f"templates/{ftype}"
        if os.path.exists(path):
            try:
                return os.listdir(path)
            except:
                return []
        return ["موقع تعليمي", "دورة تدريبية", "مستند تعريفي"]

def run_server(port=8000):
    os.chdir(os.path.dirname(os.path.abspath(__file__)))
    server = HTTPServer(('0.0.0.0', port), UnifiedServer)
    print(f"🚀 Unified Server running at http://localhost:{port}")
    print(f"📁 Serving files from: {os.getcwd()}")
    print(f"📡 API Endpoints:")
    print(f"  • GET  /api/factories")
    print(f"  • GET  /api/factory/[id]")
    print(f"  • POST /api/factory/start")
    print(f"  • GET  /api/health")
    print(f"🌐 Web Interface: http://localhost:{port}/dashboard_live.html")
    server.serve_forever()

if __name__ == "__main__":
    run_server()
