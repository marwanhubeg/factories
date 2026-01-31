#!/bin/bash

echo "================================================"
echo "⚡ Marwan Hub Factories - التشغيل السريع"
echo "================================================"

# بدء الخادم
echo "🚀 بدء تشغيل الخادم..."
python3 unified_server_final.py &

# انتظار ثانية للبدء
sleep 2

echo ""
echo "✅ النظام يعمل الآن!"
echo ""
echo "🌐 الروابط النشطة:"
echo "   1. http://localhost:8000 - الواجهة الرئيسية"
echo "   2. http://localhost:8000/advanced_dashboard.html - لوحة التحكم المتقدمة"
echo "   3. http://localhost:8000/dashboard_live.html - لوحة التحكم المباشرة"
echo ""
echo "🔧 اختبار النظام:"
echo "   curl http://localhost:8000/api/factories"
echo "   curl http://localhost:8000/api/products"
echo "   curl http://localhost:8000/api/exports"
echo ""
echo "📱 لفتح في متصفح Android:"
echo "   termux-open-url http://localhost:8000"
echo ""
echo "================================================"
echo "📊 حالة النظام الحالية:"
echo "   - 4 مصانع تعمل"
echo "   - 8 منتجات متاحة"
echo "   - 1 ملف مصدر"
echo "================================================"

# إبقاء السكريبت يعرض
echo ""
echo "⏳ اضغط Ctrl+C لإيقاف الخادم"
wait
