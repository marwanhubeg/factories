import json
import os

# قراءة حالة المصانع الحقيقية
factories_status = []
for i, ftype in enumerate(['education', 'creative', 'technology', 'corporate']):
    path = f"src/factories/{ftype}"
    exists = os.path.isdir(path)
    
    if exists:
        files = len([f for f in os.listdir(path) if f.endswith('.rs') or f.endswith('.md')])
        status = "running" if files > 0 else "maintenance"
    else:
        files = 0
        status = "error"
    
    factories_status.append({
        "id": f"factory_{i+1}",
        "type": ftype,
        "status": status,
        "files": files,
        "exists": exists
    })

print("📈 حالة المصانع الحقيقية:")
for f in factories_status:
    icon = "✅" if f['status'] == 'running' else "🟠" if f['status'] == 'maintenance' else "🔴"
    print(f"{icon} {f['id']}: {f['status']} ({f['files']} ملفات)")

# حفظ في ملف للاستخدام
with open('factories_real_status.json', 'w', encoding='utf-8') as f:
    json.dump(factories_status, f, ensure_ascii=False, indent=2)
    
print("✅ تم حفظ حالة المصانع الحقيقية")
