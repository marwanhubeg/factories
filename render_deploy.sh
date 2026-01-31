#!/bin/bash

echo "================================================"
echo "🚀 نشر Marwan Hub Factories على Render"
echo "================================================"

echo "📝 إنشاء render.yaml..."
cat << 'RENDEREOF' > render.yaml
services:
  - type: web
    name: marwan-hub-factories
    runtime: python
    buildCommand: pip install -r requirements.txt
    startCommand: python3 unified_server_final.py
    envVars:
      - key: PORT
        value: 8000
      - key: PYTHONUNBUFFERED
        value: 1
    healthCheckPath: /
    autoDeploy: true
    plan: free
RENDEREOF

# إنشاء requirements.txt
cat << 'REQEUF' > requirements.txt
# Marwan Hub Factories Requirements

# Python Standard Library
# No external dependencies required

# For future enhancements:
# Flask==2.3.3
# requests==2.31.0
# sqlite3 (built-in)
REQEUF

echo ""
echo "================================================"
echo "📋 تعليمات النشر على Render:"
echo "================================================"
echo ""
echo "1. سجل الدخول على https://render.com"
echo "2. اضغط على 'New +' ثم اختر 'Web Service'"
echo "3. اختر مستودع GitHub الخاص بك"
echo "4. أدخل المعلومات التالية:"
echo "   - Name: marwan-hub-factories"
echo "   - Environment: Python 3"
echo "   - Build Command: pip install -r requirements.txt"
echo "   - Start Command: python3 unified_server_final.py"
echo "   - Plan: Free"
echo "5. اضغط على 'Create Web Service'"
echo ""
echo "🌐 ستحصل على رابط مثل:"
echo "   https://marwan-hub-factories.onrender.com"
echo ""
echo "⏰ ملاحظة: قد يستغرق النشر الأولي 5-10 دقائق"
