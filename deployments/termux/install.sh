#!/data/data/com.termux/files/usr/bin/bash

# ألوان للتنسيق
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

clear
echo -e "${BLUE}"
echo "╔══════════════════════════════════════════════════════════╗"
echo "║                                                          ║"
echo "║  ███╗   ███╗ █████╗ ██████╗ ██╗    ██╗ █████╗ ███╗   ██╗ ║"
echo "║  ████╗ ████║██╔══██╗██╔══██╗██║    ██║██╔══██╗████╗  ██║ ║"
echo "║  ██╔████╔██║███████║██████╔╝██║ █╗ ██║███████║██╔██╗ ██║ ║"
echo "║  ██║╚██╔╝██║██╔══██║██╔══██╗██║███╗██║██╔══██║██║╚██╗██║ ║"
echo "║  ██║ ╚═╝ ██║██║  ██║██║  ██║╚███╔███╔╝██║  ██║██║ ╚████║ ║"
echo "║  ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝ ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝  ╚═══╝ ║"
echo "║                                                          ║"
echo "║            H U B   F A C T O R I E S   v3.0.0            ║"
echo "║                                                          ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo -e "${NC}"
echo "📱 تثبيت Marwan Hub على Termux"
echo "📅 التاريخ: $(date)"
echo "📦 الإصدار: v3.0.0"
echo ""

# التحقق من التحديثات
echo -e "${YELLOW}[1/8] التحقق من تحديثات Termux...${NC}"
pkg update -y && pkg upgrade -y

# تثبيت المتطلبات الأساسية
echo -e "${YELLOW}[2/8] تثبيت المتطلبات الأساسية...${NC}"
pkg install -y git curl wget rust cargo make cmake clang \
    openssl openssl-tool openssl-dev postgresql redis \
    nodejs python python-pip libxml2 libxslt

# إنشاء دليل المشروع
echo -e "${YELLOW}[3/8] إنشاء دليل المشروع...${NC}"
cd ~
if [ -d "marwan-hub-factories-v3.0.0" ]; then
    echo -e "${BLUE}⚠️  المجلد موجود مسبقاً، جاري التحديث...${NC}"
    cd marwan-hub-factories-v3.0.0
    git pull origin main
else
    git clone https://github.com/marwan-hub/factories.git marwan-hub-factories-v3.0.0
    cd marwan-hub-factories-v3.0.0
fi

# بناء المشروع
echo -e "${YELLOW}[4/8] بناء المشروع...${NC}"
cargo build --release

# إنشاء الروابط الرمزية
echo -e "${YELLOW}[5/8] إنشاء الروابط الرمزية...${NC}"
if [ ! -f "/data/data/com.termux/files/usr/bin/marwan-hub" ]; then
    ln -s ~/marwan-hub-factories-v3.0.0/target/release/marwan-hub /data/data/com.termux/files/usr/bin/marwan-hub
    chmod +x /data/data/com.termux/files/usr/bin/marwan-hub
fi

# إنشاء مجلدات البيانات
echo -e "${YELLOW}[6/8] إنشاء مجلدات البيانات...${NC}"
mkdir -p ~/.marwan-hub/{config,logs,data,templates,backups}
cp -r templates/* ~/.marwan-hub/templates/ 2>/dev/null || true

# تثبيت الخدمات
echo -e "${YELLOW}[7/8] تثبيت الخدمات...${NC}"

# خدمة PostgreSQL
if [ ! -f "$PREFIX/var/service/postgresql/run" ]; then
    echo -e "${BLUE}📊 تثبيت خدمة PostgreSQL...${NC}"
    pkg install -y postgresql
    mkdir -p $PREFIX/var/service/postgresql
    cat > $PREFIX/var/service/postgresql/run << 'SERVICE'
#!/data/data/com.termux/files/usr/bin/sh
exec postgres -D $PREFIX/var/lib/postgresql 2>&1
SERVICE
    chmod +x $PREFIX/var/service/postgresql/run
fi

# خدمة Redis
if [ ! -f "$PREFIX/var/service/redis/run" ]; then
    echo -e "${BLUE}🔴 تثبيت خدمة Redis...${NC}"
    pkg install -y redis
    mkdir -p $PREFIX/var/service/redis
    cat > $PREFIX/var/service/redis/run << 'SERVICE'
#!/data/data/com.termux/files/usr/bin/sh
exec redis-server --bind 127.0.0.1 2>&1
SERVICE
    chmod +x $PREFIX/var/service/redis/run
fi

# إنشاء ملف التكوين
echo -e "${YELLOW}[8/8] إنشاء ملف التكوين...${NC}"
cat > ~/.marwan-hub/config.toml << 'CONFIG'
[system]
name = "Marwan Hub Factories"
version = "3.0.0"
environment = "production"
data_dir = "~/.marwan-hub"

[api]
host = "0.0.0.0"
port = 8080
workers = 4
enable_cors = true
enable_swagger = true

[database]
type = "sqlite"
path = "~/.marwan-hub/data/marwan.db"
# أو استخدم PostgreSQL:
# type = "postgres"
# url = "postgresql://localhost:5432/marwan_hub"

[cache]
type = "redis"
url = "redis://127.0.0.1:6379"
ttl = 3600

[security]
jwt_secret = "change_this_in_production"
enable_auth = true
rate_limit = 100
rate_window = 60

[mhos]
version = "v2.2"
auto_optimize = true
quality_threshold = 0.85
monitoring_interval = 30

[logging]
level = "info"
file = "~/.marwan-hub/logs/marwan.log"
max_size = 100
max_files = 10

[factories]
education.enabled = true
creative.enabled = true
corporate.enabled = true
technology.enabled = true

[telemetry]
enabled = false
endpoint = "https://telemetry.marwan-hub.com"
CONFIG

# الانتهاء
echo ""
echo -e "${GREEN}✅ تم التثبيت بنجاح!${NC}"
echo ""
echo "📋 معلومات التثبيت:"
echo "   📁 المسار: ~/marwan-hub-factories-v3.0.0"
echo "   ⚙️  التكوين: ~/.marwan-hub/config.toml"
echo "   📊 البيانات: ~/.marwan-hub/data"
echo "   📝 السجلات: ~/.marwan-hub/logs"
echo ""
echo "🚀 أوامر التشغيل:"
echo "   $ marwan-hub serve           # تشغيل الخادم"
echo "   $ marwan-hub factory list    # عرض المصانع"
echo "   $ marwan-hub mhos dashboard  # لوحة MH-OS"
echo "   $ marwan-hub system health   # فحص النظام"
echo ""
echo "🔧 خدمات الخلفية:"
echo "   $ sv-enable postgresql      # تشغيل PostgreSQL"
echo "   $ sv-enable redis           # تشغيل Redis"
echo ""
echo "📖 التوثيق:"
echo "   $ marwan-hub docs           # عرض التوثيق"
echo ""
echo "🌐 الوصول عبر المتصفح:"
echo "   http://localhost:8080       # واجهة API"
echo "   http://localhost:8080/docs  # التوثيق التفاعلي"
echo ""
echo -e "${YELLOW}⚠️  ملاحظة: تأكد من تشغيل خدمات PostgreSQL و Redis قبل استخدام النظام${NC}"
