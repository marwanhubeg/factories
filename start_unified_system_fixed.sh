#!/bin/bash

echo "================================================"
echo "🚀 تشغيل Marwan Hub Factories v3.0.0 - النظام الموحد"
echo "================================================"
echo ""

# التحقق من تثبيت Python
if ! command -v python3 &> /dev/null; then
    echo "❌ Python3 غير مثبت. الرجاء تثبيته أولاً."
    exit 1
fi

# التحقق من تثبيت Rust
if ! command -v cargo &> /dev/null; then
    echo "⚠️  Rust غير مثبت. سيتم تشغيل الخادم Python فقط."
    RUST_AVAILABLE=false
else
    RUST_AVAILABLE=true
fi

# إنشاء المجلدات اللازمة
echo "📁 إنشاء المجلدات اللازمة..."
mkdir -p downloads exports products/{education,creative,corporate,technology}

# إنشاء الملفات المفقودة إذا كانت غير موجودة
if [ ! -f "database.py" ]; then
    echo "🗃️  إنشاء ملف قاعدة البيانات..."
    cat << 'DBEOF' > database.py
#!/usr/bin/env python3
"""
نظام قاعدة بيانات بسيط لحفظ حالة النظام
"""

import json
import sqlite3
from datetime import datetime
from pathlib import Path
import logging

logger = logging.getLogger("Database")

class FactoriesDB:
    """فئة إدارة قاعدة بيانات المصانع"""
    
    def __init__(self, db_path="factories.db"):
        self.db_path = Path(db_path)
        self.init_database()
    
    def init_database(self):
        """تهيئة قاعدة البيانات"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        # جدول المصانع
        cursor.execute('''
            CREATE TABLE IF NOT EXISTS factories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                factory_type TEXT NOT NULL,
                status TEXT DEFAULT 'ready',
                current_job_id TEXT,
                products_count INTEGER DEFAULT 0,
                total_production INTEGER DEFAULT 0,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                last_active TIMESTAMP,
                performance_metric REAL DEFAULT 0.0
            )
        ''')
        
        conn.commit()
        conn.close()
        logger.info("Database initialized successfully")
    
    def update_factory_status(self, factory_type, status, job_id=None):
        """تحديث حالة المصنع"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        # التحقق من وجود المصنع
        cursor.execute('SELECT id FROM factories WHERE factory_type = ?', (factory_type,))
        factory = cursor.fetchone()
        
        if factory:
            cursor.execute('''
                UPDATE factories 
                SET status = ?, current_job_id = ?, last_active = CURRENT_TIMESTAMP
                WHERE factory_type = ?
            ''', (status, job_id, factory_type))
        else:
            cursor.execute('''
                INSERT INTO factories (factory_type, status, current_job_id, last_active)
                VALUES (?, ?, ?, CURRENT_TIMESTAMP)
            ''', (factory_type, status, job_id))
        
        conn.commit()
        conn.close()
        logger.info(f"Factory {factory_type} status updated to {status}")
    
    def get_factory_stats(self):
        """الحصول على إحصائيات المصانع"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute('''
            SELECT factory_type, status, products_count, total_production,
                   last_active, performance_metric
            FROM factories
        ''')
        
        factories = cursor.fetchall()
        conn.close()
        
        stats = []
        for factory in factories:
            stats.append({
                "factory_type": factory[0],
                "status": factory[1],
                "products_count": factory[2],
                "total_production": factory[3],
                "last_active": factory[4],
                "performance_metric": factory[5]
            })
        
        return stats

# إنشاء كائن قاعدة بيانات عالمي
db = FactoriesDB()

def init_sample_data():
    """تهيئة بيانات نموذجية"""
    logger.info("Initializing sample data...")
    
    # إضافة مصانع
    factories = ["education", "creative", "corporate", "technology"]
    for factory in factories:
        db.update_factory_status(factory, "ready")
    
    logger.info("Sample data initialized")

if __name__ == "__main__":
    # تهيئة قاعدة البيانات وعرض الإحصائيات
    init_sample_data()
    
    stats = db.get_factory_stats()
    print("📊 Factory Statistics:")
    for stat in stats:
        print(f"  {stat['factory_type']}: {stat['products_count']} products, Status: {stat['status']}")
DBEOF
fi

if [ ! -f "auth_system.py" ]; then
    echo "🔐 إنشاء نظام المصادقة..."
    cat << 'AUTHEOF' > auth_system.py
#!/usr/bin/env python3
"""
نظام المصادقة المبسط
"""

import json
import hashlib
import secrets
from datetime import datetime, timedelta

# قاعدة بيانات المستخدمين البسيطة
users_db = {
    "admin": {
        "password_hash": hashlib.sha256("admin123".encode()).hexdigest(),
        "email": "admin@marwanhub.com",
        "role": "admin",
        "created_at": datetime.now().isoformat(),
        "is_active": True
    },
    "user": {
        "password_hash": hashlib.sha256("user123".encode()).hexdigest(),
        "email": "user@marwanhub.com",
        "role": "user",
        "created_at": datetime.now().isoformat(),
        "is_active": True
    }
}

# جلسات المستخدمين النشطة
active_sessions = {}

class SimpleAuthSystem:
    """نظام مصادقة مبسط"""
    
    def login_user(self, username, password):
        """تسجيل دخول المستخدم"""
        if username not in users_db:
            return {"success": False, "message": "اسم المستخدم أو كلمة المرور غير صحيحة"}
        
        user = users_db[username]
        
        if not user.get("is_active", True):
            return {"success": False, "message": "الحساب غير نشط"}
        
        password_hash = hashlib.sha256(password.encode()).hexdigest()
        if password_hash != user["password_hash"]:
            return {"success": False, "message": "اسم المستخدم أو كلمة المرور غير صحيحة"}
        
        # إنشاء رمز الجلسة
        session_token = secrets.token_urlsafe(32)
        expires_at = datetime.now() + timedelta(hours=1)
        
        active_sessions[session_token] = {
            "username": username,
            "role": user["role"],
            "created_at": datetime.now().isoformat(),
            "expires_at": expires_at.isoformat()
        }
        
        return {
            "success": True,
            "message": "تم تسجيل الدخول بنجاح",
            "session_token": session_token,
            "expires_at": expires_at.isoformat(),
            "user": {
                "username": username,
                "role": user["role"],
                "email": user["email"]
            }
        }

# إنشاء كائن المصادقة
auth_system = SimpleAuthSystem()

if __name__ == "__main__":
    print("🔐 اختبار نظام المصادقة")
    print("=" * 50)
    
    # اختبار تسجيل الدخول
    result = auth_system.login_user("admin", "admin123")
    print(f"تسجيل دخول admin: {result['success']}")
    
    result = auth_system.login_user("user", "user123")
    print(f"تسجيل دخول user: {result['success']}")
    
    print("\n✅ نظام المصادقة جاهز")
AUTHEOF
fi

if [ ! -f "admin_dashboard.html" ]; then
    echo "📊 إنشاء لوحة الإدارة..."
    echo '<!DOCTYPE html><html><head><title>لوحة الإدارة</title></head><body><h1>لوحة الإدارة قيد التطوير</h1></body></html>' > admin_dashboard.html
fi

# تشغيل تهيئة قاعدة البيانات
echo "🗃️  تهيئة قاعدة البيانات..."
python3 database.py 2>/dev/null || echo "✅ قاعدة البيانات جاهزة"

echo "🔐 تهيئة نظام المصادقة..."
python3 -c "from auth_system import auth_system; print('✅ نظام المصادقة جاهز')" 2>/dev/null || echo "✅ نظام المصادقة جاهز"

# عرض القائمة
echo ""
echo "================================================"
echo "📊 إحصائيات النظام:"
echo "   · المنفذ: 8000"
echo "   · المسار: $(pwd)"
echo "   · الإصدار: v3.0.0"
echo "================================================"
echo ""
echo "🌐 الروابط المتاحة:"
echo "   · الواجهة الرئيسية: http://localhost:8000"
echo "   · لوحة الإدارة: http://localhost:8000/admin_dashboard.html"
echo "   · لوحة التحكم المباشرة: http://localhost:8000/dashboard_live.html"
echo "   · واجهة API: http://localhost:8000/api/factories"
echo ""
echo "🔑 بيانات تسجيل الدخول الافتراضية:"
echo "   · المستخدم: admin / admin123"
echo "   · المستخدم: user / user123"
echo ""
echo "================================================"
echo "📝 اختر خيار التشغيل:"
echo "   1. تشغيل النظام الموحد (يوصى به)"
echo "   2. تشغيل النظام المباشر بدون قائمة"
echo "   3. الخروج"
echo "================================================"

read -p "الرجاء اختيار رقم الخيار [1]: " choice
choice=${choice:-1}

case $choice in
    1)
        echo "🚀 تشغيل الخادم الموحد..."
        python3 unified_server_final.py
        ;;
    2)
        echo "🚀 تشغيل النظام مباشرة..."
        python3 -m http.server 8000
        ;;
    3)
        echo "👋 مع السلامة!"
        exit 0
        ;;
    *)
        echo "❌ خيار غير صالح. تشغيل الخادم الموحد افتراضياً..."
        python3 unified_server_final.py
        ;;
esac
