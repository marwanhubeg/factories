#!/bin/bash

echo "================================================"
echo "🚀 نشر Marwan Hub Factories على Railway"
echo "================================================"

# التحقق من تثبيت Node.js و npm
if ! command -v npm &> /dev/null; then
    echo "❌ npm غير مثبت. تثبيت Node.js أولاً..."
    pkg install nodejs -y
fi

# تثبيت Railway CLI
echo "📦 تثبيت Railway CLI..."
npm install -g @railway/cli

# تسجيل الدخول
echo "🔑 تسجيل الدخول إلى Railway..."
railway login

# إنشاء ملف railway.toml
echo "📝 إنشاء تكوين Railway..."
cat << 'RAILEOF' > railway.toml
[build]
builder = "nixpacks"
buildCommand = "./build_railway.sh"

[deploy]
startCommand = "python3 unified_server_final.py"
healthcheckPath = "/"
restartPolicyType = "ON_FAILURE"

[variables]
PORT = "8000"
RAILWAY_ENVIRONMENT = "production"
