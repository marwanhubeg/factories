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
        
        # جدول المستخدمين
        cursor.execute('''
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                email TEXT,
                role TEXT DEFAULT 'user',
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                last_login TIMESTAMP,
                is_active BOOLEAN DEFAULT 1
            )
        ''')
        
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
        
        # جدول المنتجات
        cursor.execute('''
            CREATE TABLE IF NOT EXISTS products (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                factory_id INTEGER,
                factory_type TEXT NOT NULL,
                product_name TEXT NOT NULL,
                product_type TEXT,
                file_path TEXT,
                file_size INTEGER,
                quality_score REAL,
                status TEXT DEFAULT 'completed',
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                exported BOOLEAN DEFAULT 0,
                export_path TEXT,
                FOREIGN KEY (factory_id) REFERENCES factories (id)
            )
        ''')
        
        # جدول المهام
        cursor.execute('''
            CREATE TABLE IF NOT EXISTS jobs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                job_id TEXT UNIQUE NOT NULL,
                factory_type TEXT NOT NULL,
                status TEXT DEFAULT 'pending',
                product_type TEXT,
                template_used TEXT,
                user_id INTEGER,
                parameters TEXT,  -- JSON string
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                started_at TIMESTAMP,
                completed_at TIMESTAMP,
                result_path TEXT,
                FOREIGN KEY (user_id) REFERENCES users (id)
            )
        ''')
        
        # جدول التصدير
        cursor.execute('''
            CREATE TABLE IF NOT EXISTS exports (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                export_id TEXT UNIQUE NOT NULL,
                factory_type TEXT,
                product_ids TEXT,  -- JSON array
                export_format TEXT DEFAULT 'zip',
                file_path TEXT NOT NULL,
                file_size INTEGER,
                download_count INTEGER DEFAULT 0,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                expires_at TIMESTAMP,
                created_by INTEGER,
                FOREIGN KEY (created_by) REFERENCES users (id)
            )
        ''')
        
        # جدول السجل
        cursor.execute('''
            CREATE TABLE IF NOT EXISTS logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                level TEXT NOT NULL,
                module TEXT,
                message TEXT NOT NULL,
                user_id INTEGER,
                ip_address TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        ''')
        
        conn.commit()
        conn.close()
        logger.info("Database initialized successfully")
    
    def add_user(self, username, password_hash, email=None, role='user'):
        """إضافة مستخدم جديد"""
        try:
            conn = sqlite3.connect(self.db_path)
            cursor = conn.cursor()
            
            cursor.execute('''
                INSERT INTO users (username, password_hash, email, role)
                VALUES (?, ?, ?, ?)
            ''', (username, password_hash, email, role))
            
            conn.commit()
            user_id = cursor.lastrowid
            conn.close()
            
            logger.info(f"User added: {username}")
            return user_id
        except sqlite3.IntegrityError:
            logger.error(f"User {username} already exists")
            return None
    
    def authenticate_user(self, username, password_hash):
        """مصادقة المستخدم"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute('''
            SELECT id, username, role FROM users 
            WHERE username = ? AND password_hash = ? AND is_active = 1
        ''', (username, password_hash))
        
        user = cursor.fetchone()
        conn.close()
        
        if user:
            self.update_last_login(user[0])
            logger.info(f"User authenticated: {username}")
            return {
                "id": user[0],
                "username": user[1],
                "role": user[2]
            }
        return None
    
    def update_last_login(self, user_id):
        """تحديث وقت آخر تسجيل دخول"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute('''
            UPDATE users SET last_login = CURRENT_TIMESTAMP
            WHERE id = ?
        ''', (user_id,))
        
        conn.commit()
        conn.close()
    
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
    
    def add_product(self, factory_type, product_name, product_type, file_path, file_size, quality_score=1.0):
        """إضافة منتج جديد"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        # الحصول على معرف المصنع
        cursor.execute('SELECT id FROM factories WHERE factory_type = ?', (factory_type,))
        factory = cursor.fetchone()
        factory_id = factory[0] if factory else None
        
        cursor.execute('''
            INSERT INTO products 
            (factory_id, factory_type, product_name, product_type, file_path, file_size, quality_score)
            VALUES (?, ?, ?, ?, ?, ?, ?)
        ''', (factory_id, factory_type, product_name, product_type, str(file_path), file_size, quality_score))
        
        # تحديث عدد المنتجات
        if factory_id:
            cursor.execute('''
                UPDATE factories 
                SET products_count = products_count + 1,
                    total_production = total_production + 1,
                    last_active = CURRENT_TIMESTAMP
                WHERE id = ?
            ''', (factory_id,))
        
        conn.commit()
        product_id = cursor.lastrowid
        conn.close()
        
        logger.info(f"Product added: {product_name} from {factory_type}")
        return product_id
    
    def create_job(self, job_id, factory_type, product_type, user_id=None, parameters=None):
        """إنشاء مهمة جديدة"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        params_json = json.dumps(parameters) if parameters else '{}'
        
        cursor.execute('''
            INSERT INTO jobs 
            (job_id, factory_type, product_type, user_id, parameters, started_at)
            VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
        ''', (job_id, factory_type, product_type, user_id, params_json))
        
        conn.commit()
        conn.close()
        
        logger.info(f"Job created: {job_id} for {factory_type}")
        return job_id
    
    def complete_job(self, job_id, result_path):
        """إكمال مهمة"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute('''
            UPDATE jobs 
            SET status = 'completed',
                completed_at = CURRENT_TIMESTAMP,
                result_path = ?
            WHERE job_id = ?
        ''', (result_path, job_id))
        
        conn.commit()
        conn.close()
        
        logger.info(f"Job completed: {job_id}")
    
    def add_export(self, export_id, factory_type, product_ids, file_path, file_size, created_by=None):
        """إضافة سجل تصدير"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        product_ids_json = json.dumps(product_ids)
        
        cursor.execute('''
            INSERT INTO exports 
            (export_id, factory_type, product_ids, file_path, file_size, created_by)
            VALUES (?, ?, ?, ?, ?, ?)
        ''', (export_id, factory_type, product_ids_json, str(file_path), file_size, created_by))
        
        conn.commit()
        conn.close()
        
        logger.info(f"Export added: {export_id}")
    
    def log_activity(self, level, message, module=None, user_id=None, ip_address=None):
        """تسجيل نشاط"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute('''
            INSERT INTO logs (level, module, message, user_id, ip_address)
            VALUES (?, ?, ?, ?, ?)
        ''', (level, module, message, user_id, ip_address))
        
        conn.commit()
        conn.close()
    
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
    
    def get_recent_products(self, limit=10):
        """الحصول على أحدث المنتجات"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute('''
            SELECT factory_type, product_name, product_type, file_size, quality_score, created_at
            FROM products
            ORDER BY created_at DESC
            LIMIT ?
        ''', (limit,))
        
        products = cursor.fetchall()
        conn.close()
        
        return [
            {
                "factory": p[0],
                "name": p[1],
                "type": p[2],
                "size": p[3],
                "quality": p[4],
                "created": p[5]
            }
            for p in products
        ]
    
    def export_to_json(self, output_file="database_backup.json"):
        """تصدير قاعدة البيانات إلى JSON"""
        data = {
            "exported_at": datetime.now().isoformat(),
            "factories": self.get_factory_stats(),
            "recent_products": self.get_recent_products(50)
        }
        
        with open(output_file, 'w', encoding='utf-8') as f:
            json.dump(data, f, ensure_ascii=False, indent=2)
        
        logger.info(f"Database exported to {output_file}")
        return output_file

# إنشاء كائن قاعدة بيانات عالمي
db = FactoriesDB()

# وظائف مساعدة للاستخدام
def init_sample_data():
    """تهيئة بيانات نموذجية"""
    logger.info("Initializing sample data...")
    
    # إضافة مستخدمين
    db.add_user("admin", "admin123", "admin@marwanhub.com", "admin")
    db.add_user("user1", "user123", "user1@marwanhub.com", "user")
    
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
