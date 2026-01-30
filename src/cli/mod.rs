pub mod commands;

use clap::{Parser, Subcommand};
use std::sync::Arc;
use crate::core::factory_manager::FactoryManager;

/// واجهة سطر أوامر Marwan Hub Factories
#[derive(Parser)]
#[command(
    name = "marwan-hub",
    version = "3.0.0",
    author = "Marwan Hub Team",
    about = "نظام المصانع الذكية للتعليم والإبداع والتقنية",
    long_about = "Marwan Hub Factories v3.0.0 - نظام متكامل لإدارة مصانع المحتوى الذكية"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    #[arg(short, long, help = "تفعيل وضع التصحيح")]
    pub debug: bool,
    
    #[arg(short, long, help = "مسار ملف التكوين")]
    pub config: Option<String>,
}

/// الأوامر المتاحة
#[derive(Subcommand)]
pub enum Commands {
    /// تشغيل خادم API
    Serve {
        #[arg(short, long, default_value = "8080", help = "رقم المنفذ")]
        port: u16,
        
        #[arg(long, help = "عنوان الربط")]
        host: Option<String>,
    },
    
    /// إدارة المصانع
    Factory {
        #[command(subcommand)]
        subcommand: FactoryCommands,
    },
    
    /// عمليات التصنيع
    Manufacture {
        #[arg(help = "نوع المصنع")]
        factory_type: String,
        
        #[arg(help = "مدخلات التصنيع (JSON)")]
        input: String,
        
        #[arg(short, long, help = "معلمات إضافية (JSON)")]
        params: Option<String>,
    },
    
    /// نظام MH-OS
    Mhos {
        #[command(subcommand)]
        subcommand: MhosCommands,
    },
    
    /// توليد المحتوى
    Generate {
        #[arg(help = "اسم القالب")]
        template: String,
        
        #[arg(short, long, help = "معلمات التوليد (JSON)")]
        params: Option<String>,
        
        #[arg(short, long, help = "لغة المحتوى")]
        lang: Option<String>,
    },
    
    /// التحليل والتقييم
    Analyze {
        #[arg(help = "المحتوى المراد تحليله")]
        content: String,
        
        #[arg(short, long, help = "نوع التحليل")]
        analysis_type: Option<String>,
    },
    
    /// إدارة النظام
    System {
        #[command(subcommand)]
        subcommand: SystemCommands,
    },
    
    /// المساعدة والتوثيق
    Docs {
        #[arg(help = "موضوع التوثيق")]
        topic: Option<String>,
    },
}

/// أوامر المصانع
#[derive(Subcommand)]
pub enum FactoryCommands {
    /// سرد جميع المصانع
    List,
    
    /// إنشاء مصنع جديد
    Create {
        #[arg(help = "نوع المصنع")]
        factory_type: String,
        
        #[arg(help = "اسم المصنع")]
        name: String,
        
        #[arg(short, long, help = "تكوين المصنع (JSON)")]
        config: Option<String>,
    },
    
    /// عرض معلومات مصنع
    Info {
        #[arg(help = "نوع المصنع")]
        factory_type: String,
    },
    
    /// تحديث مصنع
    Update {
        #[arg(help = "نوع المصنع")]
        factory_type: String,
        
        #[arg(help = "التحديثات (JSON)")]
        updates: String,
    },
    
    /// حذف مصنع
    Delete {
        #[arg(help = "نوع المصنع")]
        factory_type: String,
    },
}

/// أوامر MH-OS
#[derive(Subcommand)]
pub enum MhosCommands {
    /// عرض لوحة التحكم
    Dashboard,
    
    /// فحص بوابات الجودة
    QualityGates,
    
    /// تحسين النظام
    Optimize,
    
    /// تحليل الأداء
    Analyze,
}

/// أوامر النظام
#[derive(Subcommand)]
pub enum SystemCommands {
    /// فحص صحة النظام
    Health,
    
    /// عرض إحصائيات النظام
    Stats,
    
    /// إعادة تشغيل النظام
    Restart,
    
    /// النسخ الاحتياطي
    Backup {
        #[arg(short, long, help = "مسار النسخة الاحتياطية")]
        path: Option<String>,
    },
    
    /// استعادة النسخة الاحتياطية
    Restore {
        #[arg(help = "مسار ملف الاستعادة")]
        backup_file: String,
    },
}

/// تنفيذ CLI
pub async fn run_cli() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let factory_manager = Arc::new(FactoryManager::new());
    
    match cli.command {
        Commands::Serve { port, host } => {
            commands::serve::execute(factory_manager, port, host).await
        }
        Commands::Factory { subcommand } => {
            commands::factory::execute(factory_manager, subcommand).await
        }
        Commands::Manufacture { factory_type, input, params } => {
            commands::manufacture::execute(factory_manager, factory_type, input, params).await
        }
        Commands::Mhos { subcommand } => {
            commands::mhos::execute(factory_manager, subcommand).await
        }
        Commands::Generate { template, params, lang } => {
            commands::generate::execute(factory_manager, template, params, lang).await
        }
        Commands::Analyze { content, analysis_type } => {
            commands::analyze::execute(factory_manager, content, analysis_type).await
        }
        Commands::System { subcommand } => {
            commands::system::execute(factory_manager, subcommand).await
        }
        Commands::Docs { topic } => {
            commands::docs::execute(topic).await
        }
    }
}
