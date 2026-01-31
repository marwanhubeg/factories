#!/bin/bash

echo "======================================================"
echo "🏭 نظام Marwan Hub Factories v3.0.0 - التشغيل الكامل"
echo "======================================================"

# 1. التحقق من الملفات
echo "🔍 جاري التحقق من ملفات النظام..."

if [ -f "api_server.py" ]; then
    echo "✅ API Server موجود"
else
    echo "❌ API Server غير موجود"
    exit 1
fi

if [ -f "dashboard_live.html" ]; then
    echo "✅ لوحة التحكم موجودة"
else
    echo "❌ لوحة التحكم غير موجودة"
    exit 1
fi

# 2. تشغيل API Server
echo ""
echo "🚀 جاري تشغيل خادم API..."
python3 api_server.py &
API_PID=$!
sleep 2
echo "✅ API Server يعمل (PID: $API_PID)"

# 3. تشغيل خادم الويب للواجهة
echo ""
echo "🌐 جاري تشغيل خادم الويب..."
python3 -m http.server 8000 &
WEB_PID=$!
sleep 1
echo "✅ خادم الويب يعمل (PID: $WEB_PID)"

# 4. فتح الواجهة
echo ""
echo "📊 فتح لوحة التحكم..."
termux-open-url "http://localhost:8000/dashboard_live.html" 2>/dev/null

# 5. عرض معلومات النظام
echo ""
echo "======================================================"
echo "✅ النظام يعمل بنجاح!"
echo ""
echo "📌 روابط النظام:"
echo "   لوحة التحكم:      http://localhost:8000/dashboard_live.html"
echo "   API Server:       http://localhost:8081/api/factories"
echo "   الصفحة الرئيسية: http://localhost:8000/"
echo ""
echo "📊 المصانع المتاحة:"
echo "   1. 🎓 مصنع التعليم     - src/factories/education/"
echo "   2. 🎨 مصنع الإبداع     - src/factories/creative/"
echo "   3. 💻 مصنع التقنية     - src/factories/technology/"
echo "   4. 📊 مصنع الشركات     - src/factories/corporate/"
echo ""
echo "⚡ الأوامر:"
echo "   kill $API_PID $WEB_PID  # إيقاف النظام"
echo "   ./run_complete_system.sh # إعادة التشغيل"
echo ""
echo "======================================================"
echo "🎉 تهانينا! النظام يعرض المصانع الحقيقية الآن!"
echo "======================================================"

# الانتظار للإغلاق
wait
