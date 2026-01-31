#!/bin/bash

echo "================================================"
echo "🚀 MARWAN HUB FACTORIES v3.0.0 - التشغيل الكامل"
echo "================================================"

# 1. تشغيل خادم الويب
echo "🌐 جاري تشغيل خادم الويب..."
python3 -m http.server 8000 &
WEB_PID=$!
echo "✅ الخادم يعمل (PID: $WEB_PID)"

# 2. فتح لوحة التحكم
echo "📊 فتح لوحة التحكم..."
termux-open-url "http://localhost:8000/dashboard_live.html" 2>/dev/null

# 3. عرض روابط التشغيل
echo ""
echo "📌 روابط النظام:"
echo "   المحلي: http://localhost:8000/dashboard_live.html"
echo "   الإنترنت: https://marwanhubeg.github.io/factories/"
echo ""

# 4. عرض المصانع المتاحة
echo "🏭 المصانع الجاهزة:"
echo "   1. 🎓 مصنع التعليم - templates/education/"
echo "   2. 🎨 مصنع الإبداع - templates/creative/"
echo "   3. 💻 مصنع التقنية - templates/technology/"
echo "   4. 📊 مصنع الشركات - templates/corporate/"
echo ""

# 5. أوامر CLI وهمية (للعرض)
echo "⚡ أوامر سريعة:"
echo "   • ./mh-cli factory list        # عرض المصانع"
echo "   • ./mh-cli factory start all   # تشغيل جميع المصانع"
echo "   • ./mh-cli docs generate       # إنشاء وثائق"
echo ""

# 6. إنشاء واجهة CLI وهمية للعرض
cat << 'PYTHON' > mh-cli
#!/usr/bin/env python3
import sys
import json

def main():
    if len(sys.argv) < 2:
        print("🚀 Marwan Hub CLI v3.0.0")
        print("الأوامر:")
        print("  factory list    - عرض المصانع")
        print("  factory start   - تشغيل مصنع")
        print("  docs generate   - إنشاء وثائق")
        return
    
    command = sys.argv[1]
    
    if command == "factory" and len(sys.argv) > 2:
        if sys.argv[2] == "list":
            factories = [
                {"id": 1, "name": "مصنع التعليم", "status": "✅ نشط"},
                {"id": 2, "name": "مصنع الإبداع", "status": "✅ نشط"},
                {"id": 3, "name": "مصنع التقنية", "status": "✅ نشط"},
                {"id": 4, "name": "مصنع الشركات", "status": "✅ نشط"},
            ]
            print("📋 المصانع المتاحة:")
            for f in factories:
                print(f"  {f['id']}. {f['name']} - {f['status']}")
    
    elif command == "docs" and len(sys.argv) > 2:
        if sys.argv[2] == "generate":
            print("📄 جاري إنشاء وثائق النظام...")
            print("✅ تم إنشاء: README.md, API_DOCS.md, USER_GUIDE.md")

if __name__ == "__main__":
    main()
PYTHON

chmod +x mh-cli

echo "✅ تم إنشاء واجهة CLI وهمية: ./mh-cli"
echo ""
echo "🔧 للتحكم بالنظام:"
echo "   kill $WEB_PID     # إيقاف الخادم"
echo "   ./mh-cli          # عرض الأوامر"
echo "================================================"
