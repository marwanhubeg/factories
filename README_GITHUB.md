# 🏭 Marwan Hub Factories v3.0.0

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Actix](https://img.shields.io/badge/Actix-000000?style=for-the-badge&logo=actix&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white)
![Kubernetes](https://img.shields.io/badge/Kubernetes-326CE5?style=for-the-badge&logo=kubernetes&logoColor=white)
![GitHub](https://img.shields.io/badge/GitHub-181717?style=for-the-badge&logo=github&logoColor=white)

## 🌐 الروابط السريعة

| الرابط | الوصف |
|--------|-------|
| [🚀 لوحة التحكم](https://marwanhubeg.github.io/factories/) | لوحة التحكم الرئيسية |
| [📖 التوثيق](https://marwanhubeg.github.io/factories/docs/) | التوثيق الكامل |
| [🐙 GitHub](https://github.com/marwanhubeg/factories) | المستودع الرئيسي |
| [🐛 المشكلات](https://github.com/marwanhubeg/factories/issues) | الإبلاغ عن مشكلات |
| [💬 المناقشات](https://github.com/marwanhubeg/factories/discussions) | مجتمع المطورين |
| [📦 Releases](https://github.com/marwanhubeg/factories/releases) | الإصدارات |

## 🎯 نظرة عامة

**Marwan Hub Factories** هو نظام متكامل لإدارة مصانع المحتوى الذكية، مكتوب بلغة Rust مع واجهات API، CLI، ونظام MH-OS الذكي.

### ✨ المميزات الرئيسية
- **🏭 4 أنواع مصانع**: تعليم، إبداع، شركات، تقنية
- **🤖 MH-OS v2.2**: نظام تشغيل ذكي للمصانع
- **🔧 واجهات متعددة**: REST API، CLI، ويب
- **📦 نشر مرن**: Docker، Kubernetes، AWS، Termux
- **🇸🇦 دعم عربي**: واجهات وتوثيق عربي كامل

## 🚀 البدء السريع

### التثبيت
```bash
# باستخدام Cargo
cargo install marwan-hub-factories

# أو تحميل الملف التنفيذي
curl -L https://github.com/marwanhubeg/factories/releases/download/v3.0.0/marwan-hub-linux-x64.tar.gz | tar xz

# تشغيل الخادم
marwan-hub serve --port 8080

# عرض المصانع
marwan-hub factory list

# إنشاء مصنع تعليمي
marwan-hub factory create education "مدرسة الذكاء"

# تصنيع محتوى
marwan-hub manufacture education '{"topic": "تعلم الآلة"}'

factories/
├── src/                    # الكود المصدر (Rust)
├── docs/                   # التوثيق
├── templates/              # القوالب الجاهزة
├── deployments/            # ملفات النشر
├── tests/                  # الاختبارات
└── .github/               # تكوينات GitHub
