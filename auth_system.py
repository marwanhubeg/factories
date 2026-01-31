#!/usr/bin/env python3
"""
نظام المصادقة المتكامل للمستخدمين
"""

import json
import hashlib
import secrets
import time
from datetime import datetime, timedelta
from functools import wraps
import logging

logger = logging.getLogger("AuthSystem")

# قاعدة بيانات المستخدمين البسيطة (في الإنتاج استخدم قاعدة بيانات حقيقية)
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

class AuthSystem:
    """فئة نظام المصادقة"""
    
    def __init__(self, db_file="auth.db"):
        self.db_file = db_file
        self.session_timeout = 3600  # ساعة واحدة بالثواني
    
    def hash_password(self, password, salt=None):
        """تجزئة كلمة المرور مع الملح"""
        if salt is None:
            salt = secrets.token_hex(16)
        
        password_hash = hashlib.sha256(f"{password}{salt}".encode()).hexdigest()
        return password_hash, salt
    
    def verify_password(self, password, stored_hash, salt):
        """التحقق من كلمة المرور"""
        password_hash, _ = self.hash_password(password, salt)
        return password_hash == stored_hash
    
    def register_user(self, username, password, email, role="user"):
        """تسجيل مستخدم جديد"""
        if username in users_db:
            return {"success": False, "message": "اسم المستخدم موجود بالفعل"}
        
        password_hash, salt = self.hash_password(password)
        
        users_db[username] = {
            "password_hash": password_hash,
            "salt": salt,
            "email": email,
            "role": role,
            "created_at": datetime.now().isoformat(),
            "is_active": True
        }
        
        logger.info(f"User registered: {username}")
        return {"success": True, "message": "تم تسجيل المستخدم بنجاح"}
    
    def login_user(self, username, password):
        """تسجيل دخول المستخدم"""
        if username not in users_db:
            return {"success": False, "message": "اسم المستخدم أو كلمة المرور غير صحيحة"}
        
        user = users_db[username]
        
        if not user.get("is_active", True):
            return {"success": False, "message": "الحساب غير نشط"}
        
        if not self.verify_password(password, user["password_hash"], user.get("salt", "")):
            return {"success": False, "message": "اسم المستخدم أو كلمة المرور غير صحيحة"}
        
        # إنشاء رمز الجلسة
        session_token = secrets.token_urlsafe(32)
        expires_at = datetime.now() + timedelta(seconds=self.session_timeout)
        
        active_sessions[session_token] = {
            "username": username,
            "role": user["role"],
            "created_at": datetime.now().isoformat(),
            "expires_at": expires_at.isoformat()
        }
        
        logger.info(f"User logged in: {username}")
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
    
    def logout_user(self, session_token):
        """تسجيل خروج المستخدم"""
        if session_token in active_sessions:
            username = active_sessions[session_token]["username"]
            del active_sessions[session_token]
            logger.info(f"User logged out: {username}")
            return {"success": True, "message": "تم تسجيل الخروج بنجاح"}
        
        return {"success": False, "message": "جلسة غير صالحة"}
    
    def verify_session(self, session_token):
        """التحقق من صحة الجلسة"""
        if session_token not in active_sessions:
            return {"valid": False, "message": "جلسة غير صالحة"}
        
        session = active_sessions[session_token]
        expires_at = datetime.fromisoformat(session["expires_at"])
        
        if datetime.now() > expires_at:
            del active_sessions[session_token]
            return {"valid": False, "message": "انتهت صلاحية الجلسة"}
        
        # تجديد الجلسة
        new_expires_at = datetime.now() + timedelta(seconds=self.session_timeout)
        session["expires_at"] = new_expires_at.isoformat()
        
        return {
            "valid": True,
            "username": session["username"],
            "role": session["role"],
            "expires_at": new_expires_at.isoformat()
        }
    
    def require_auth(self, required_role=None):
        """مزخرف يتطلب المصادقة"""
        def decorator(func):
            @wraps(func)
            def wrapper(request_handler, *args, **kwargs):
                # الحصول على رمز الجلسة من الرؤوس
                auth_header = request_handler.headers.get('Authorization')
                
                if not auth_header or not auth_header.startswith('Bearer '):
                    request_handler.send_error(401, "مصادقة مطلوبة")
                    return
                
                session_token = auth_header.split(' ')[1]
                session_info = self.verify_session(session_token)
                
                if not session_info["valid"]:
                    request_handler.send_error(401, "جلسة غير صالحة")
                    return
                
                # التحقق من الصلاحية
                if required_role and session_info["role"] != required_role:
                    request_handler.send_error(403, "صلاحية غير كافية")
                    return
                
                # تمرير معلومات المستخدم للدالة
                kwargs['user_info'] = session_info
                return func(request_handler, *args, **kwargs)
            return wrapper
        return decorator
    
    def get_user_stats(self):
        """الحصول على إحصائيات المستخدمين"""
        total_users = len(users_db)
        active_sessions_count = len(active_sessions)
        
        roles = {}
        for user in users_db.values():
            role = user.get("role", "user")
            roles[role] = roles.get(role, 0) + 1
        
        return {
            "total_users": total_users,
            "active_sessions": active_sessions_count,
            "roles_distribution": roles
        }
    
    def export_users(self):
        """تصدير بيانات المستخدمين (بدون كلمات المرور)"""
        export_data = []
        
        for username, user_data in users_db.items():
            export_data.append({
                "username": username,
                "email": user_data.get("email"),
                "role": user_data.get("role"),
                "created_at": user_data.get("created_at"),
                "is_active": user_data.get("is_active", True)
            })
        
        return export_data

# إنشاء كائن المصادقة العام
auth_system = AuthSystem()

# واجهات API للمصادقة
def handle_auth_api(path, method, body=None, headers=None):
    """معالجة طلبات API المصادقة"""
    
    if path == "/api/auth/register" and method == "POST":
        return handle_register(body)
    
    elif path == "/api/auth/login" and method == "POST":
        return handle_login(body)
    
    elif path == "/api/auth/logout" and method == "POST":
        return handle_logout(body, headers)
    
    elif path == "/api/auth/verify" and method == "GET":
        return handle_verify(headers)
    
    elif path == "/api/auth/stats" and method == "GET":
        return handle_stats(headers)
    
    else:
        return {"success": False, "message": "نقطة نهاية غير صالحة"}

def handle_register(body):
    """معالجة تسجيل مستخدم جديد"""
    try:
        data = json.loads(body)
        username = data.get("username")
        password = data.get("password")
        email = data.get("email")
        role = data.get("role", "user")
        
        if not all([username, password, email]):
            return {"success": False, "message": "جميع الحقول مطلوبة"}
        
        result = auth_system.register_user(username, password, email, role)
        return result
        
    except json.JSONDecodeError:
        return {"success": False, "message": "بيانات غير صالحة"}
    except Exception as e:
        logger.error(f"Registration error: {e}")
        return {"success": False, "message": "حدث خطأ أثناء التسجيل"}

def handle_login(body):
    """معالجة تسجيل الدخول"""
    try:
        data = json.loads(body)
        username = data.get("username")
        password = data.get("password")
        
        if not all([username, password]):
            return {"success": False, "message": "اسم المستخدم وكلمة المرور مطلوبان"}
        
        result = auth_system.login_user(username, password)
        return result
        
    except json.JSONDecodeError:
        return {"success": False, "message": "بيانات غير صالحة"}
    except Exception as e:
        logger.error(f"Login error: {e}")
        return {"success": False, "message": "حدث خطأ أثناء تسجيل الدخول"}

def handle_logout(body, headers):
    """معالجة تسجيل الخروج"""
    try:
        auth_header = headers.get("Authorization") if headers else None
        
        if auth_header and auth_header.startswith("Bearer "):
            session_token = auth_header.split(" ")[1]
        else:
            data = json.loads(body) if body else {}
            session_token = data.get("session_token")
        
        if not session_token:
            return {"success": False, "message": "رمز الجلسة مطلوب"}
        
        result = auth_system.logout_user(session_token)
        return result
        
    except Exception as e:
        logger.error(f"Logout error: {e}")
        return {"success": False, "message": "حدث خطأ أثناء تسجيل الخروج"}

def handle_verify(headers):
    """معالجة التحقق من الجلسة"""
    try:
        auth_header = headers.get("Authorization") if headers else None
        
        if not auth_header or not auth_header.startswith("Bearer "):
            return {"success": False, "message": "رأس المصادقة مطلوب"}
        
        session_token = auth_header.split(" ")[1]
        session_info = auth_system.verify_session(session_token)
        
        if session_info["valid"]:
            return {
                "success": True,
                "valid": True,
                "user": {
                    "username": session_info["username"],
                    "role": session_info["role"]
                },
                "expires_at": session_info["expires_at"]
            }
        else:
            return {"success": False, "valid": False, "message": session_info["message"]}
            
    except Exception as e:
        logger.error(f"Verification error: {e}")
        return {"success": False, "valid": False, "message": "حدث خطأ أثناء التحقق"}

def handle_stats(headers):
    """معالجة إحصائيات النظام"""
    try:
        # التحقق من الصلاحية (للمسؤولين فقط)
        verify_result = handle_verify(headers)
        if not verify_result.get("success") or verify_result.get("role") != "admin":
            return {"success": False, "message": "صلاحية غير كافية"}
        
        stats = auth_system.get_user_stats()
        users = auth_system.export_users()
        
        return {
            "success": True,
            "stats": stats,
            "users": users
        }
        
    except Exception as e:
        logger.error(f"Stats error: {e}")
        return {"success": False, "message": "حدث خطأ أثناء جلب الإحصائيات"}

if __name__ == "__main__":
    # اختبار النظام
    print("🔐 اختبار نظام المصادقة")
    print("=" * 50)
    
    # تسجيل مستخدم جديد
    print("1. تسجيل مستخدم جديد:")
    result = handle_register(json.dumps({
        "username": "testuser",
        "password": "test123",
        "email": "test@marwanhub.com"
    }))
    print(f"   النتيجة: {result}")
    
    # تسجيل الدخول
    print("\n2. تسجيل الدخول:")
    result = handle_login(json.dumps({
        "username": "testuser",
        "password": "test123"
    }))
    print(f"   النتيجة: {result}")
    
    if result.get("success"):
        session_token = result.get("session_token")
        
        # التحقق من الجلسة
        print("\n3. التحقق من الجلسة:")
        headers = {"Authorization": f"Bearer {session_token}"}
        result = handle_verify(headers)
        print(f"   النتيجة: {result}")
        
        # تسجيل الخروج
        print("\n4. تسجيل الخروج:")
        result = handle_logout(None, headers)
        print(f"   النتيجة: {result}")
    
    print("\n" + "=" * 50)
    print("✅ اختبار النظام اكتمل")
