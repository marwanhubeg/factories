import os
import json

print("🔍 فحص حالة المصانع...")
factories = []

for ftype in ['education', 'creative', 'technology', 'corporate']:
    path = f"src/factories/{ftype}"
    exists = os.path.exists(path)
    files = 0
    
    if exists:
        files = len([f for f in os.listdir(path) if f.endswith('.rs') or f.endswith('.md')])
    
    status = "✅ نشط" if exists and files > 0 else "⚠️ يحتاج إصلاح"
    
    factories.append({
        "type": ftype,
        "exists": exists,
        "files": files,
        "status": status
    })
    
    print(f"🏭 {ftype}: {status} ({files} ملفات)")

print("\n🎯 نتيجة الفحص:")
for f in factories:
    if f['files'] == 0:
        print(f"❌ {f['type']}: يحتاج إنشاء ملفات")
    elif not f['exists']:
        print(f"❌ {f['type']}: المجلد غير موجود")
    else:
        print(f"✅ {f['type']}: جاهز للعمل")
