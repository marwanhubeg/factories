#!/bin/bash

echo "======================================================"
echo "🚀 MARWAN HUB FACTORIES - التشغيل النهائي"
echo "======================================================"

# تنظيف أي خوادم قديمة
echo "🧹 جاري تنظيف الخوادم القديمة..."
pkill -f "python3" 2>/dev/null
sleep 2

# تشغيل الخادم الموحد
echo ""
echo "🌐 جاري تشغيل الخادم الموحد..."
echo "📊 سيعمل على: http://localhost:8000"
echo ""

python3 unified_server.py &
SERVER_PID=$!

sleep 3

echo ""
echo "✅ الخادم يعمل! (PID: $SERVER_PID)"
echo ""
echo "📌 روابط مهمة:"
echo "   • لوحة التحكم: http://localhost:8000/dashboard_live.html"
echo "   • API المصانع: http://localhost:8000/api/factories"
echo "   • الصفحة الرئيسية: http://localhost:8000/"
echo ""
echo "🔍 اختبار الاتصال:"
curl -s http://localhost:8000/api/health | python3 -m json.tool || echo "⚠️ جاري التحميل..."

echo ""
echo "======================================================"
echo "🎉 النظام يعمل بنجاح! افتح لوحة التحكم الآن!"
echo "======================================================"

# فتح المتصفح تلقائياً
termux-open-url "http://localhost:8000/dashboard_live.html" 2>/dev/null

# الانتظار
echo "Press Ctrl+C to stop the server..."
wait
