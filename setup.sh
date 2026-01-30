#!/bin/bash

# Marwan Hub Factories v3.0.0 - Setup Script
# نسخة التثبيت والتكوين التلقائي

set -e

echo "🚀 بدء إعداد Marwan Hub Factories v3.0.0"
echo "========================================"

# التحقق من وجود Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust غير مثبت. جاري التثبيت..."
    
    if [[ "$OSTYPE" == "linux-android" ]]; then
        # Termux على Android
        pkg install rust -y
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source $HOME/.cargo/env
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        brew install rust
    else
        echo "⚠️  نظام غير معروف. يرجى تثبيت Rust يدوياً: https://rustup.rs"
        exit 1
    fi
fi

echo "✅ Rust مثبت: $(rustc --version)"

# إنشاء مجلدات التخزين
echo "📁 إنشاء مجلدات النظام..."
mkdir -p {data,outputs,logs,templates,config}

# نسخ ملفات التكوين
echo "⚙️  إعداد ملفات التكوين..."
if [ ! -f config/config.toml ]; then
    cat > config/config.toml << 'CONFIG'
[system]
name = "Marwan Hub Factories"
version = "3.0.0"
environment = "production"
log_level = "info"

[database]
url = "sqlite:data/marwan_hub.db"
pool_size = 5
connect_timeout = 30

[server]
host = "0.0.0.0"
port = 8080
workers = 4
max_body_size = 10485760

[github]
enabled = true
api_url = "https://api.github.com"
timeout = 30

[factories]
education.enabled = true
creative.enabled = true
corporate.enabled = true
technology.enabled = true

[mhos]
version = "2.2.0"
enable_dashboard = true
quality_gates_enabled = true
auto_validation = true
CONFIG
    echo "✅ تم إنشاء config.toml"
fi

# إعداد قاعدة البيانات
echo "🗄️  إعداد قاعدة البيانات..."
if [ ! -f data/marwan_hub.db ]; then
    echo "جاري إنشاء قاعدة البيانات الجديدة..."
    sqlite3 data/marwan_hub.db << 'SQL'
CREATE TABLE IF NOT EXISTS factories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    status TEXT DEFAULT 'active',
    config TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY,
    factory_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    status TEXT DEFAULT 'pending',
    metadata TEXT,
    output_path TEXT,
    github_url TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP,
    FOREIGN KEY (factory_id) REFERENCES factories(id)
);

CREATE TABLE IF NOT EXISTS quality_gates (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    gate_type TEXT NOT NULL,
    status TEXT DEFAULT 'pending',
    score INTEGER,
    details TEXT,
    checked_at TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

CREATE TABLE IF NOT EXISTS system_metrics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    metric_name TEXT NOT NULL,
    metric_value REAL NOT NULL,
    recorded_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO factories (id, name, type, status) VALUES 
('edu_001', 'مصنع التعليم', 'education', 'active'),
('cre_001', 'مصنع الإبداع', 'creative', 'active'),
('cor_001', 'مصنع المؤسسات', 'corporate', 'active'),
('tec_001', 'مصنع التقنية', 'technology', 'active');
SQL
    echo "✅ تم إنشاء قاعدة البيانات"
fi

# بناء المشروع
echo "🏗️  بناء المشروع..."
cargo build --release

# إعداد ملفات التوثيق
echo "📖 إعداد التوثيق..."
if [ ! -f docs/quick-start.md ]; then
    mkdir -p docs
    cat > docs/quick-start.md << 'DOC'
# البدء السريع

## التشغيل السريع
```bash
# تشغيل الخادم
./target/release/marwan-hub-factories

# أو باستخدام Docker
docker-compose up -d
```

الأوامر الأساسية

```bash
# عرض المساعدة
./target/release/mh-cli --help

# إنشاء موقع ويب
./target/release/mh-cli website create --name "موقعي"

# إدارة المصانع
./target/release/mh-cli factory list
```

الواجهة

· 🌐 API: http://localhost:8080
· 📊 Dashboard: http://localhost:8080/dashboard
· 📖 التوثيق: http://localhost:8080/docs
  DOC
  echo "✅ تم إنشاء التوثيق"
  fi

إعداد systemd service (لـ Linux)

if [[ "$OSTYPE" == "linux-gnu"* ]] && [ -d /etc/systemd/system ]; then
echo "⚙️  إعداد systemd service..."
sudo tee /etc/systemd/system/marwan-hub-factories.service > /dev/null << 'SERVICE'
[Unit]
Description=Marwan Hub Factories v3.0.0
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$HOME/marwan-hub-factories-v3.0.0
ExecStart=$HOME/marwan-hub-factories-v3.0.0/target/release/marwan-hub-factories
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=marwan-hub-factories
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
SERVICE

fi

إعداد Termux (لـ Android)

if [[ "$OSTYPE" == "linux-android" ]]; then
echo "📱 إعداد Termux..."

Marwan Hub Aliases

alias mh-start='cd $HOME/marwan-hub-factories-v3.0.0 && cargo run --bin marwan-hub-factories'
alias mh-cli='cd $HOME/marwan-hub-factories-v3.0.0 && cargo run --bin mh-cli --'
alias mh-status='curl -s http://localhost:8080/api/status | python -m json.tool'
alias mh-website='cd $HOME/marwan-hub-factories-v3.0.0 && cargo run --bin mh-cli -- website create'
BASHRC

fi

إنشاء ملف .env

echo "🔧 إنشاء ملف .env..."
cat > .env << 'ENV'

Marwan Hub Factories Environment

DATABASE_URL=sqlite:data/marwan_hub.db
RUST_LOG=info
MHOS_VERSION=2.2.0
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
GITHUB_API_TOKEN=
ENABLE_HTTPS=false
MAX_UPLOAD_SIZE=10485760
ENV

echo "🎉 تم الانتهاء من الإعداد!"
echo ""
echo "📋 الخطوات التالية:"
echo "1. قم بتعديل ملف .env إذا لزم الأمر"
echo "2. قم بتشغيل النظام: cargo run --release"
echo "3. افتح المتصفح: http://localhost:8080"
echo ""
echo "🔧 للأوامر السريعة:"
echo "   ./target/release/mh-cli --help"
echo "   docker-compose up -d"
echo ""
echo "📞 للدعم: support@marwanhub.com"
