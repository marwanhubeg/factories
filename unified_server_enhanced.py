#!/usr/bin/env python3
"""
خادم متقدم مع نظام تحكم منطقي في المصانع
"""
from http.server import HTTPServer, SimpleHTTPRequestHandler
import json
import os
import urllib.parse
import uuid
from datetime import datetime, timedelta
import random

class EnhancedServer(SimpleHTTPRequestHandler):
    # تخزين البيانات في الذاكرة
    production_requests = []
    factory_schedules = {}
    quality_tests = {}
    factory_metrics = {}
    
    def do_GET(self):
        # API endpoints
        if self.path.startswith('/api/'):
            self.handle_api()
            return
        
        # ملفات عادية
        super().do_GET()
    
    def handle_api(self):
        # المصانع الأساسية
        if self.path == '/api/factories':
            self.send_json_response(self.get_factories())
        
        elif self.path == '/api/health':
            self.send_json_response({
                "status": "ok",
                "version": "3.1.0",
                "features": ["production", "scheduling", "quality", "monitoring"],
                "timestamp": datetime.now().isoformat()
            })
        
        elif self.path.startswith('/api/factory/'):
            parts = self.path.split('/')
            if len(parts) == 4:
                factory_id = parts[3]
                self.send_json_response(self.get_factory_details(factory_id))
            else:
                self.send_error(404)
        
        # طلبات الإنتاج
        elif self.path == '/api/production/requests':
            self.send_json_response(self.production_requests)
        
        elif self.path.startswith('/api/production/request/'):
            request_id = self.path.split('/')[-1]
            request = next((r for r in self.production_requests if r['id'] == request_id), None)
            if request:
                self.send_json_response(request)
            else:
                self.send_error(404)
        
        # الجدولة
        elif self.path == '/api/schedules':
            self.send_json_response(self.factory_schedules)
        
        # اختبارات الجودة
        elif self.path == '/api/quality/tests':
            self.send_json_response(self.quality_tests)
        
        # المقاييس
        elif self.path == '/api/metrics':
            self.send_json_response(self.get_live_metrics())
        
        else:
            self.send_error(404)
    
    def do_POST(self):
        content_length = int(self.headers['Content-Length'])
        post_data = self.rfile.read(content_length)
        data = json.loads(post_data)
        
        if self.path == '/api/production/create':
            response = self.create_production_request(data)
            self.send_json_response(response)
        
        elif self.path == '/api/schedule/create':
            response = self.create_schedule(data)
            self.send_json_response(response)
        
        elif self.path == '/api/quality/test':
            response = self.run_quality_test(data)
            self.send_json_response(response)
        
        elif self.path == '/api/factory/start':
            response = self.start_factory(data)
            self.send_json_response(response)
        
        elif self.path == '/api/factory/stop':
            response = self.stop_factory(data)
            self.send_json_response(response)
        
        elif self.path == '/api/production/approve':
            response = self.approve_production(data)
            self.send_json_response(response)
        
        else:
            self.send_error(404)
    
    def do_DELETE(self):
        if self.path.startswith('/api/production/request/'):
            request_id = self.path.split('/')[-1]
            self.production_requests = [r for r in self.production_requests if r['id'] != request_id]
            self.send_json_response({"success": True, "message": "تم حذف الطلب"})
        else:
            self.send_error(404)
    
    # ========== دوال مساعدة ==========
    
    def send_json_response(self, data):
        self.send_response(200)
        self.send_header('Content-Type', 'application/json; charset=utf-8')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()
        self.wfile.write(json.dumps(data, ensure_ascii=False, indent=2).encode('utf-8'))
    
    def get_factories(self):
        factories = []
        types = ['education', 'creative', 'technology', 'corporate']
        
        for i, ftype in enumerate(types):
            path = f"src/factories/{ftype}"
            exists = os.path.isdir(path)
            
            # مقاييس عشوائية للمحاكاة
            metrics = self.factory_metrics.get(f"factory_{i+1}", {
                "efficiency": random.randint(70, 98),
                "quality_score": random.randint(80, 100),
                "utilization": random.randint(60, 95),
                "downtime": random.randint(0, 5),
                "production_rate": random.randint(50, 200)
            })
            
            factories.append({
                "id": f"factory_{i+1}",
                "name": self.get_factory_name(ftype),
                "type": ftype,
                "status": self.get_factory_status(f"factory_{i+1}"),
                "capacity": random.randint(100, 1000),
                "current_load": random.randint(0, 100),
                "metrics": metrics,
                "health": "excellent" if metrics['efficiency'] > 85 else "good" if metrics['efficiency'] > 70 else "warning",
                "last_maintenance": (datetime.now() - timedelta(days=random.randint(0, 30))).strftime("%Y-%m-%d"),
                "next_maintenance": (datetime.now() + timedelta(days=random.randint(1, 60))).strftime("%Y-%m-%d")
            })
        
        return factories
    
    def get_factory_details(self, factory_id):
        factories = self.get_factories()
        factory = next((f for f in factories if f['id'] == factory_id), None)
        
        if factory:
            # إضافة تفاصيل إضافية
            factory['production_history'] = self.generate_production_history()
            factory['quality_reports'] = self.generate_quality_reports()
            factory['upcoming_tasks'] = self.generate_upcoming_tasks(factory_id)
            factory['alerts'] = self.generate_alerts(factory_id)
        
        return factory or {"error": "المصنع غير موجود"}
    
    def create_production_request(self, data):
        request_id = str(uuid.uuid4())[:8]
        
        request = {
            "id": request_id,
            "factory_id": data.get('factory_id'),
            "product_type": data.get('product_type', 'عام'),
            "quantity": data.get('quantity', 1),
            "priority": data.get('priority', 'medium'),  # low, medium, high, urgent
            "status": "pending",  # pending, approved, in_progress, completed, cancelled
            "created_at": datetime.now().isoformat(),
            "estimated_completion": (datetime.now() + timedelta(hours=random.randint(1, 24))).isoformat(),
            "quality_requirements": data.get('quality_requirements', {}),
            "notes": data.get('notes', ''),
            "created_by": data.get('user', 'system')
        }
        
        self.production_requests.append(request)
        
        # تحديث مقاييس المصنع
        factory_id = request['factory_id']
        if factory_id not in self.factory_metrics:
            self.factory_metrics[factory_id] = {}
        
        return {
            "success": True,
            "message": f"✅ تم إنشاء طلب إنتاج #{request_id}",
            "request_id": request_id,
            "request": request
        }
    
    def create_schedule(self, data):
        schedule_id = str(uuid.uuid4())[:8]
        factory_id = data.get('factory_id')
        
        schedule = {
            "id": schedule_id,
            "factory_id": factory_id,
            "start_time": data.get('start_time'),
            "end_time": data.get('end_time'),
            "shift_type": data.get('shift_type', 'normal'),  # normal, night, overtime
            "tasks": data.get('tasks', []),
            "workforce": data.get('workforce', 1),
            "energy_consumption": random.randint(50, 200),
            "status": "scheduled"
        }
        
        if factory_id not in self.factory_schedules:
            self.factory_schedules[factory_id] = []
        
        self.factory_schedules[factory_id].append(schedule)
        
        return {
            "success": True,
            "message": f"✅ تم جدولة المصنع {factory_id}",
            "schedule_id": schedule_id
        }
    
    def run_quality_test(self, data):
        test_id = str(uuid.uuid4())[:8]
        
        # محاكاة اختبار الجودة
        test_results = {
            "id": test_id,
            "factory_id": data.get('factory_id'),
            "product_sample": data.get('product_sample', 'A1'),
            "test_type": data.get('test_type', 'standard'),
            "parameters_tested": data.get('parameters', ['durability', 'accuracy', 'performance']),
            "results": {
                "durability": random.randint(85, 100),
                "accuracy": random.randint(90, 100),
                "performance": random.randint(88, 100),
                "safety": random.randint(95, 100)
            },
            "overall_score": 0,
            "status": "passed",  # passed, failed, needs_review
            "tested_at": datetime.now().isoformat(),
            "tester": data.get('tester', 'auto_system'),
            "notes": data.get('notes', '')
        }
        
        # حساب النتيجة العامة
        scores = list(test_results['results'].values())
        test_results['overall_score'] = sum(scores) // len(scores)
        test_results['status'] = 'passed' if test_results['overall_score'] >= 90 else 'needs_review' if test_results['overall_score'] >= 80 else 'failed'
        
        self.quality_tests[test_id] = test_results
        
        return {
            "success": True,
            "message": f"✅ تم إجراء اختبار الجودة #{test_id}",
            "test_id": test_id,
            "results": test_results
        }
    
    def start_factory(self, data):
        factory_id = data.get('factory_id')
        
        return {
            "success": True,
            "message": f"🚀 تم تشغيل المصنع {factory_id}",
            "factory_id": factory_id,
            "started_at": datetime.now().isoformat(),
            "estimated_readiness": (datetime.now() + timedelta(minutes=5)).isoformat(),
            "current_operations": random.randint(1, 10)
        }
    
    def stop_factory(self, data):
        factory_id = data.get('factory_id')
        
        return {
            "success": True,
            "message": f"🛑 تم إيقاف المصنع {factory_id}",
            "factory_id": factory_id,
            "stopped_at": datetime.now().isoformat(),
            "downtime_reason": data.get('reason', 'صيانة روتينية'),
            "next_start": (datetime.now() + timedelta(hours=8)).isoformat()
        }
    
    def approve_production(self, data):
        request_id = data.get('request_id')
        
        for request in self.production_requests:
            if request['id'] == request_id:
                request['status'] = 'approved'
                request['approved_at'] = datetime.now().isoformat()
                request['approved_by'] = data.get('approver', 'admin')
                
                return {
                    "success": True,
                    "message": f"✅ تم اعتماد طلب الإنتاج #{request_id}",
                    "request": request
                }
        
        return {"success": False, "message": "الطلب غير موجود"}
    
    def get_live_metrics(self):
        return {
            "total_factories": 4,
            "active_factories": random.randint(2, 4),
            "total_production_requests": len(self.production_requests),
            "pending_requests": len([r for r in self.production_requests if r['status'] == 'pending']),
            "completed_today": random.randint(10, 50),
            "avg_efficiency": random.randint(75, 95),
            "quality_compliance": random.randint(85, 99),
            "energy_consumption": random.randint(500, 2000),
            "alerts": random.randint(0, 3),
            "timestamp": datetime.now().isoformat()
        }
    
    # ========== دوال توليد بيانات للمحاكاة ==========
    
    def get_factory_name(self, ftype):
        names = {
            'education': 'مصنع التعليم الذكي',
            'creative': 'مصنع الإبداع الرقمي',
            'technology': 'مصنع التقنية المتقدمة',
            'corporate': 'مصنع حلول الأعمال'
        }
        return names.get(ftype, ftype)
    
    def get_factory_status(self, factory_id):
        statuses = ['running', 'idle', 'maintenance', 'error']
        return random.choice(statuses)
    
    def generate_production_history(self):
        history = []
        for i in range(7):
            date = (datetime.now() - timedelta(days=i)).strftime("%Y-%m-%d")
            history.append({
                "date": date,
                "units_produced": random.randint(50, 200),
                "defects": random.randint(0, 5),
                "efficiency": random.randint(75, 98),
                "energy_used": random.randint(100, 500)
            })
        return history
    
    def generate_quality_reports(self):
        reports = []
        for i in range(5):
            reports.append({
                "id": f"QR{i+1}",
                "date": (datetime.now() - timedelta(days=random.randint(1, 30))).strftime("%Y-%m-%d"),
                "score": random.randint(85, 100),
                "inspector": f"مفتش {random.choice(['أ', 'ب', 'ج'])}",
                "notes": random.choice(['جيد جداً', 'ممتاز', 'يحتاج تحسين بسيط', 'مطابق للمواصفات'])
            })
        return reports
    
    def generate_upcoming_tasks(self, factory_id):
        tasks = []
        for i in range(3):
            tasks.append({
                "id": f"TASK{i+1}",
                "type": random.choice(['maintenance', 'upgrade', 'inspection', 'calibration']),
                "scheduled_date": (datetime.now() + timedelta(days=random.randint(1, 14))).strftime("%Y-%m-%d"),
                "duration_hours": random.randint(2, 8),
                "priority": random.choice(['low', 'medium', 'high']),
                "assigned_to": random.choice(['فريق الصيانة', 'فريق الجودة', 'فريق التقنية'])
            })
        return tasks
    
    def generate_alerts(self, factory_id):
        alerts = []
        alert_types = [
            ("⚠️", "تحذير", "ارتفاع درجة الحرارة", "medium"),
            ("🔧", "صيانة", "مطلوب صيانة روتينية", "low"),
            ("📊", "أداء", "انخفاض في الكفاءة", "medium"),
            ("✅", "معلومات", "اكتمال الدفعة", "low")
        ]
        
        for i in range(random.randint(0, 2)):
            icon, category, message, severity = random.choice(alert_types)
            alerts.append({
                "id": f"ALERT{i+1}",
                "icon": icon,
                "category": category,
                "message": message,
                "severity": severity,
                "time": (datetime.now() - timedelta(hours=random.randint(1, 24))).strftime("%H:%M"),
                "acknowledged": random.choice([True, False])
            })
        
        return alerts

def run_server(port=8000):
    os.chdir(os.path.dirname(os.path.abspath(__file__)))
    server = HTTPServer(('0.0.0.0', port), EnhancedServer)
    
    print("\n" + "="*60)
    print("🚀 MARWAN HUB FACTORIES v3.1.0 - نظام التحكم المتقدم")
    print("="*60)
    print(f"📡 الخادم يعمل على: http://localhost:{port}")
    print("\n📋 نقاط الوصول المتاحة:")
    print("  🌐 الواجهة: /dashboard_advanced.html")
    print("  📊 المصانع: /api/factories")
    print("  📋 الطلبات: /api/production/requests")
    print("  🕒 الجداول: /api/schedules")
    print("  🧪 الجودة: /api/quality/tests")
    print("  📈 المقاييس: /api/metrics")
    print("\n⚡ المميزات الجديدة:")
    print("  • نظام طلبات الإنتاج")
    print("  • جدولة المصانع الذكية")
    print("  • اختبارات جوية متقدمة")
    print("  • مراقبة في الوقت الحقيقي")
    print("="*60 + "\n")
    
    server.serve_forever()

if __name__ == "__main__":
    run_server()

    # ========== نقاط الوصول الجديدة للتصدير ==========
    
    def handle_export_api(self):
        if self.path == '/api/export/products':
            self.send_json_response(self.list_available_products())
        
        elif self.path.startswith('/api/export/download/'):
            product_id = self.path.split('/')[-1]
            self.download_product(product_id)
        
        elif self.path.startswith('/api/export/batch/'):
            factory_type = self.path.split('/')[-1]
            self.export_batch(factory_type)
        
        elif self.path == '/api/export/create':
            content_length = int(self.headers['Content-Length'])
            post_data = self.rfile.read(content_length)
            data = json.loads(post_data)
            self.create_product(data)
        
        else:
            self.send_error(404)
    
    def list_available_products(self):
        """قائمة المنتجات المتاحة للتصدير"""
        products = []
        
        for ftype in ['education', 'creative', 'technology', 'corporate']:
            products_dir = f"products/{ftype}"
            if os.path.exists(products_dir):
                for file in os.listdir(products_dir):
                    if file.endswith(('.json', '.txt', '.md')):
                        filepath = f"{products_dir}/{file}"
                        stat = os.stat(filepath)
                        
                        products.append({
                            "id": file.split('.')[0],
                            "name": file,
                            "type": ftype,
                            "size": stat.st_size,
                            "created": datetime.fromtimestamp(stat.st_ctime).isoformat(),
                            "modified": datetime.fromtimestamp(stat.st_mtime).isoformat(),
                            "download_url": f"/download/{ftype}/{file}"
                        })
        
        return {
            "count": len(products),
            "products": products,
            "timestamp": datetime.now().isoformat()
        }
    
    def download_product(self, product_id):
        """تنزيل منتج معين"""
        # البحث عن المنتج
        for ftype in ['education', 'creative', 'technology', 'corporate']:
            products_dir = f"products/{ftype}"
            if os.path.exists(products_dir):
                for file in os.listdir(products_dir):
                    if product_id in file:
                        filepath = f"{products_dir}/{file}"
                        
                        # إرسال الملف
                        self.send_response(200)
                        self.send_header('Content-Type', 'application/octet-stream')
                        self.send_header('Content-Disposition', f'attachment; filename="{file}"')
                        
                        # قراءة وإرسال الملف
                        with open(filepath, 'rb') as f:
                            content = f.read()
                            self.send_header('Content-Length', str(len(content)))
                            self.end_headers()
                            self.wfile.write(content)
                        return
        
        self.send_error(404, "المنتج غير موجود")
    
    def export_batch(self, factory_type):
        """تصدير دفعة من منتجات مصنع"""
        import zipfile
        import io
        
        products_dir = f"products/{factory_type}"
        if not os.path.exists(products_dir):
            self.send_error(404, "لا توجد منتجات")
            return
        
        # إنشاء ملف ZIP في الذاكرة
        zip_buffer = io.BytesIO()
        with zipfile.ZipFile(zip_buffer, 'w') as zipf:
            for file in os.listdir(products_dir):
                filepath = f"{products_dir}/{file}"
                zipf.write(filepath, file)
        
        # إرسال ملف ZIP
        zip_buffer.seek(0)
        self.send_response(200)
        self.send_header('Content-Type', 'application/zip')
        self.send_header('Content-Disposition', f'attachment; filename="{factory_type}_products.zip"')
        self.send_header('Content-Length', str(zip_buffer.getbuffer().nbytes))
        self.end_headers()
        self.wfile.write(zip_buffer.read())
    
    def create_product(self, data):
        """إنشاء منتج جديد"""
        factory_type = data.get('factory_type')
        product_name = data.get('name')
        content = data.get('content')
        format_type = data.get('format', 'json')
        
        if not all([factory_type, product_name, content]):
            self.send_error(400, "بيانات ناقصة")
            return
        
        # إنشاء المنتج
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        product_id = f"{factory_type}_{timestamp}"
        filename = f"{product_id}.{format_type}"
        filepath = f"products/{factory_type}/{filename}"
        
        # إنشاء المجلد إذا لم يكن موجوداً
        os.makedirs(f"products/{factory_type}", exist_ok=True)
        
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
        
        elif format_type in ['txt', 'md']:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
        
        self.send_json_response({
            "success": True,
            "message": f"تم إنشاء المنتج {product_name}",
            "product_id": product_id,
            "download_url": f"/api/export/download/{product_id}",
            "filepath": filepath
        })

# تحديث handle_api ليشمل التصدير
    def handle_api(self):
        # إضافة التصدير
        if self.path.startswith('/api/export'):
            self.handle_export_api()
            return
        
        # باقي API السابق...

    # ========== نقاط الوصول الجديدة للتصدير ==========
    
    def handle_export_api(self):
        if self.path == '/api/export/products':
            self.send_json_response(self.list_available_products())
        
        elif self.path.startswith('/api/export/download/'):
            product_id = self.path.split('/')[-1]
            self.download_product(product_id)
        
        elif self.path.startswith('/api/export/batch/'):
            factory_type = self.path.split('/')[-1]
            self.export_batch(factory_type)
        
        elif self.path == '/api/export/create':
            content_length = int(self.headers['Content-Length'])
            post_data = self.rfile.read(content_length)
            data = json.loads(post_data)
            self.create_product(data)
        
        else:
            self.send_error(404)
    
    def list_available_products(self):
        """قائمة المنتجات المتاحة للتصدير"""
        products = []
        
        for ftype in ['education', 'creative', 'technology', 'corporate']:
            products_dir = f"products/{ftype}"
            if os.path.exists(products_dir):
                for file in os.listdir(products_dir):
                    if file.endswith(('.json', '.txt', '.md')):
                        filepath = f"{products_dir}/{file}"
                        stat = os.stat(filepath)
                        
                        products.append({
                            "id": file.split('.')[0],
                            "name": file,
                            "type": ftype,
                            "size": stat.st_size,
                            "created": datetime.fromtimestamp(stat.st_ctime).isoformat(),
                            "modified": datetime.fromtimestamp(stat.st_mtime).isoformat(),
                            "download_url": f"/download/{ftype}/{file}"
                        })
        
        return {
            "count": len(products),
            "products": products,
            "timestamp": datetime.now().isoformat()
        }
    
    def download_product(self, product_id):
        """تنزيل منتج معين"""
        # البحث عن المنتج
        for ftype in ['education', 'creative', 'technology', 'corporate']:
            products_dir = f"products/{ftype}"
            if os.path.exists(products_dir):
                for file in os.listdir(products_dir):
                    if product_id in file:
                        filepath = f"{products_dir}/{file}"
                        
                        # إرسال الملف
                        self.send_response(200)
                        self.send_header('Content-Type', 'application/octet-stream')
                        self.send_header('Content-Disposition', f'attachment; filename="{file}"')
                        
                        # قراءة وإرسال الملف
                        with open(filepath, 'rb') as f:
                            content = f.read()
                            self.send_header('Content-Length', str(len(content)))
                            self.end_headers()
                            self.wfile.write(content)
                        return
        
        self.send_error(404, "المنتج غير موجود")
    
    def export_batch(self, factory_type):
        """تصدير دفعة من منتجات مصنع"""
        import zipfile
        import io
        
        products_dir = f"products/{factory_type}"
        if not os.path.exists(products_dir):
            self.send_error(404, "لا توجد منتجات")
            return
        
        # إنشاء ملف ZIP في الذاكرة
        zip_buffer = io.BytesIO()
        with zipfile.ZipFile(zip_buffer, 'w') as zipf:
            for file in os.listdir(products_dir):
                filepath = f"{products_dir}/{file}"
                zipf.write(filepath, file)
        
        # إرسال ملف ZIP
        zip_buffer.seek(0)
        self.send_response(200)
        self.send_header('Content-Type', 'application/zip')
        self.send_header('Content-Disposition', f'attachment; filename="{factory_type}_products.zip"')
        self.send_header('Content-Length', str(zip_buffer.getbuffer().nbytes))
        self.end_headers()
        self.wfile.write(zip_buffer.read())
    
    def create_product(self, data):
        """إنشاء منتج جديد"""
        factory_type = data.get('factory_type')
        product_name = data.get('name')
        content = data.get('content')
        format_type = data.get('format', 'json')
        
        if not all([factory_type, product_name, content]):
            self.send_error(400, "بيانات ناقصة")
            return
        
        # إنشاء المنتج
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        product_id = f"{factory_type}_{timestamp}"
        filename = f"{product_id}.{format_type}"
        filepath = f"products/{factory_type}/{filename}"
        
        # إنشاء المجلد إذا لم يكن موجوداً
        os.makedirs(f"products/{factory_type}", exist_ok=True)
        
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
        
        elif format_type in ['txt', 'md']:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
        
        self.send_json_response({
            "success": True,
            "message": f"تم إنشاء المنتج {product_name}",
            "product_id": product_id,
            "download_url": f"/api/export/download/{product_id}",
            "filepath": filepath
        })

# تحديث handle_api ليشمل التصدير
    def handle_api(self):
        # إضافة التصدير
        if self.path.startswith('/api/export'):
            self.handle_export_api()
            return
        
        # باقي API السابق...
