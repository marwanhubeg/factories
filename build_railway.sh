#!/bin/bash
echo "🚀 بناء Marwan Hub Factories v3.0.0..."
echo "📦 تثبيت Python..."
apt-get update && apt-get install -y python3 python3-pip
echo "📁 إنشاء المجلدات..."
mkdir -p products/{education,creative,corporate,technology} exports downloads
echo "✅ البناء اكتمل!"
