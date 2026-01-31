#!/bin/bash

echo "================================================"
echo "🚀 نشر Marwan Hub Factories v3.0.0"
echo "================================================"
echo ""

# الألوان للواجهة
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# دالة لعرض الرسائل
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# التحقق من الملفات
check_files() {
    print_info "التحقق من ملفات النظام..."
    
    required_files=(
        "unified_server_final.py"
        "database.py"
        "auth_system.py"
        "admin_dashboard.html"
        "Dockerfile.final"
        "README_FINAL.md"
        "USER_GUIDE_AR.md"
    )
    
    missing_files=()
    
    for file in "${required_files[@]}"; do
        if [ ! -f "$file" ]; then
            missing_files+=("$file")
        fi
    done
    
    if [ ${#missing_files[@]} -gt 0 ]; then
        print_error "الملفات التالية مفقودة:"
        for file in "${missing_files[@]}"; do
            echo "  - $file"
        done
        return 1
    else
        print_success "جميع الملفات موجودة"
        return 0
    fi
}

# تحديث GitHub
update_github() {
    print_info "تحديث مستودع GitHub..."
    
    # التحقق من وجود git
    if ! command -v git &> /dev/null; then
        print_error "Git غير مثبت"
        return 1
    fi
    
    # التحقق من كوننا في مستودع git
    if [ ! -d ".git" ]; then
        print_error "هذا المجلد ليس مستودع git"
        return 1
    fi
    
    # إضافة جميع الملفات
    git add .
    
    # Commit التغييرات
    git commit -m "🚀 Marwan Hub Factories v3.0.0 - النظام الموحد"
    
    # Push إلى GitHub
    if git push origin main; then
        print_success "تم تحديث GitHub بنجاح"
        return 0
    else
        print_error "فشل في تحديث GitHub"
        return 1
    fi
}

# النشر على Railway
deploy_railway() {
    print_info "النشر على Railway..."
    
    # التحقق من تثبيت Railway CLI
    if ! command -v railway &> /dev/null; then
        print_warning "Railway CLI غير مثبت"
        read -p "هل تريد تثبيته؟ (y/n): " install_railway
        
        if [ "$install_railway" = "y" ]; then
            npm install -g @railway/cli
        else
            print_warning "تخطي النشر على Railway"
            return 0
        fi
    fi
    
    # التحقق من تسجيل الدخول
    if ! railway status &> /dev/null; then
        print_info "يجب تسجيل الدخول إلى Railway"
        railway login
    fi
    
    # النشر
    if railway up; then
        print_success "تم النشر على Railway بنجاح"
        railway open
        return 0
    else
        print_error "فشل في النشر على Railway"
        return 1
    fi
}

# النشر على Render
deploy_render() {
    print_info "النشر على Render..."
    
    print_info "تعليمات النشر على Render:"
    echo ""
    echo "1. سجل الدخول على https://render.com"
    echo "2. اضغط على 'New Web Service'"
    echo "3. اختر 'Build and deploy from a Git repository'"
    echo "4. اختر مستودع GitHub الخاص بك"
    echo "5. أدخل المعلومات التالية:"
    echo "   - Name: marwan-hub-factories"
    echo "   - Root Directory: ."
    echo "   - Environment: Docker"
    echo "   - Dockerfile Path: ./Dockerfile.final"
    echo "   - Plan: Free"
    echo "6. اضغط على 'Create Web Service'"
    echo ""
    
    read -p "هل قمت بالنشر على Render؟ (y/n): " deployed
    
    if [ "$deployed" = "y" ]; then
        print_success "تم النشر على Render بنجاح"
        return 0
    else
        print_warning "تخطي النشر على Render"
        return 0
    fi
}

# النشر على Fly.io
deploy_flyio() {
    print_info "النشر على Fly.io..."
    
    # التحقق من تثبيت Fly CLI
    if ! command -v fly &> /dev/null; then
        print_warning "Fly CLI غير مثبت"
        read -p "هل تريد تثبيته؟ (y/n): " install_fly
        
        if [ "$install_fly" = "y" ]; then
            curl -L https://fly.io/install.sh | sh
        else
            print_warning "تخطي النشر على Fly.io"
            return 0
        fi
    fi
    
    # التحقق من تسجيل الدخول
    if ! fly auth whoami &> /dev/null; then
        print_info "يجب تسجيل الدخول إلى Fly.io"
        fly auth login
    fi
    
    # النشر
    if fly launch --now; then
        print_success "تم النشر على Fly.io بنجاح"
        fly open
        return 0
    else
        print_error "فشل في النشر على Fly.io"
        return 1
    fi
}

# إنشاء فيديو توضيحي
create_demo_video() {
    print_info "إنشاء دليل فيديو توضيحي..."
    
    cat << 'VIDEOEOF' > DEMO_VIDEO_GUIDE.md
# 🎥 دليل إنشاء فيديو توضيحي

## 📋 محتوى الفيديو

### الجزء 1: المقدمة (30 ثانية)
- تقديم المشروع
- عرض المميزات الرئيسية
- الهدف من المشروع

### الجزء 2: التثبيت (1 دقيقة)
- استنساخ المشروع من GitHub
- تشغيل النظام محلياً
- فتح المتصفح

### الجزء 3: واجهة المستخدم (2 دقيقة)
- تسجيل الدخول
- شرح لوحة الإدارة
- عرض أنواع المصانع

### الجزء 4: الإنتاج (2 دقيقة)
- بدء تشغيل مصنع التعليم
- إنتاج محتوى تعليمي
- مراقبة عملية الإنتاج

### الجزء 5: التصدير (1 دقيقة)
- تصدير المنتجات
- تنزيل الملفات
- فتح الملفات المصدرة

### الجزء 6: النشر (1 دقيقة)
- النشر على Railway
- فتح الرابط المنشور
- اختبار النظام على الإنترنت

### الجزء 7: الخاتمة (30 ثانية)
- تلخيص المميزات
- دعوة للمشاركة
- معلومات التواصل

## 🛠️ الأدوات المطلوبة
1. برنامج تسجيل الشاشة (OBS Studio)
2. محرر نصي (VS Code)
3. متصفح (Chrome/Firefox)
4. ميكروفون (للتسجيل الصوتي)

## 🎬 نصائح للتسجيل
1. استخدم دقة 1080p
2. سجل بصوت واضح
3. استخدم مؤشر الفأرة الكبير
4. تجنب الخلفيات المشتتة
5. أضف موسيقى خلفية هادئة

## 📤 رفع الفيديو
1. YouTube: للمستخدمين العامين
2. LinkedIn: للمحترفين
3. Twitter: للإعلانات السريعة
4. GitHub: في قسم Releases

## 🏷️ الوسوم المقترحة
#MarwanHub #Factories #OpenSource #Rust #Python #Automation

## 📝 النص النهائي
[يمكنك استخدام النص الموجود في README_FINAL.md]
VIDEOEOF

    print_success "تم إنشاء دليل الفيديو التوضيحي"
}

# دالة رئيسية
main() {
    echo ""
    echo "================================================"
    echo "🎯 اختر طريقة النشر:"
    echo "   1. تحديث GitHub فقط"
    echo "   2. النشر على Railway"
    echo "   3. النشر على Render"
    echo "   4. النشر على Fly.io"
    echo "   5. النشر على جميع المنصات"
    echo "   6. إنشاء فيديو توضيحي"
    echo "   7. الخروج"
    echo "================================================"
    
    read -p "الرجاء اختيار رقم الخيار [1]: " choice
    choice=${choice:-1}
    
    case $choice in
        1)
            check_files && update_github
            ;;
        2)
            check_files && update_github && deploy_railway
            ;;
        3)
            check_files && update_github && deploy_render
            ;;
        4)
            check_files && update_github && deploy_flyio
            ;;
        5)
            check_files && update_github && deploy_railway && deploy_render && deploy_flyio
            ;;
        6)
            create_demo_video
            ;;
        7)
            print_info "مع السلامة!"
            exit 0
            ;;
        *)
            print_error "خيار غير صالح"
            exit 1
            ;;
    esac
    
    # عرض الروابط النهائية
    if [ $? -eq 0 ]; then
        echo ""
        echo "================================================"
        print_success "🎉 تم اكتمال العملية بنجاح!"
        echo "================================================"
        echo ""
        echo "📚 الوثائق:"
        echo "   
          · دليل المستخدم: USER_GUIDE_AR.md"
        echo "   · دليل الفيديو: DEMO_VIDEO_GUIDE.md"
        echo ""
        echo "🌐 روابط مفيدة:"
        echo "   · مستودع GitHub: https://github.com/marwanhubeg/factories"
        echo "   · دليل المساهمة: CONTRIBUTING.md"
        echo ""
        echo "📞 للدعم:"
        echo "   · GitHub Issues: https://github.com/marwanhubeg/factories/issues"
        echo "   · البريد: support@marwanhub.com"
        echo ""
    fi
}

# تشغيل الدالة الرئيسية
main
