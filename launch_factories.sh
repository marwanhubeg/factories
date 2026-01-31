#!/bin/bash
echo "🚀 إطلاق Marwan Hub Factories v3.0.0"
echo "====================================="

# 1. تشغيل لوحة التحكم
echo "🌐 تشغيل لوحة التحكم..."
if command -v python3 &> /dev/null; then
    python3 -m http.server 8000 &
    echo "✅ لوحة التحكم: http://localhost:8000"
    echo "✅ لوحة التحكم الحية: http://localhost:8000/dashboard_live.html"
fi

# 2. عرض القوالب الجاهزة
echo "📋 القوالب المتاحة:"
ls templates/ 2>/dev/null || echo "  • موقع تعليمي"
echo "  • دورة تدريبية"
echo "  • مستند تقني"
echo "  • تصميم إبداعي"

# 3. إنشاء مثال سريع
cat << 'HTML' > factory_example.html
<!DOCTYPE html>
<html>
<head>
    <title>مصنع تعليمي - Marwan Hub</title>
    <style>
        body { font-family: Arial; margin: 40px; background: #f5f5f5; }
        .container { max-width: 800px; margin: auto; background: white; padding: 20px; border-radius: 10px; }
        .factory { background: #e3f2fd; padding: 15px; margin: 10px; border-radius: 5px; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🎓 مصنع التعليم - Marwan Hub</h1>
        <div class="factory">
            <h3>منتج: دورة الذكاء الاصطناعي</h3>
            <p>✅ جاهز للإنتاج</p>
            <button onclick="produce()">إنتاج الآن</button>
        </div>
        <script>
            function produce() {
                alert("🎉 تم إنتاج دورة تعليمية جديدة!");
            }
        </script>
    </div>
</body>
</html>
HTML

echo "✅ تم إنشاء مثال مصنع في factory_example.html"
echo "====================================="
echo "📞 للدعم: https://github.com/marwanhubeg/factories"
