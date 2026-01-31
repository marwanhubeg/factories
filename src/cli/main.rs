use clap::Parser;
use marwan_hub_factories::cli::{Cli, run_cli};
use marwan_hub_factories::core::factory_manager::FactoryManager;
use std::sync::Arc;
use std::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // تحميل متغيرات البيئة
    dotenv::dotenv().ok();
    
    // إعداد التسجيل
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format(|buf, record| {
            use chrono::Local;
            use env_logger::fmt::Color;
            use std::io::Write;
            
            let mut style = buf.style();
            let color = match record.level() {
                log::Level::Error => Color::Red,
                log::Level::Warn => Color::Yellow,
                log::Level::Info => Color::Green,
                log::Level::Debug => Color::Blue,
                log::Level::Trace => Color::Cyan,
            };
            
            style.set_color(color).set_bold(true);
            
            writeln!(
                buf,
                "{} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                style.value(record.level()),
                record.args()
            )
        })
        .init();
    
    // عرض شعار النظام
    print_banner();
    
    // تشغيل CLI
    if let Err(e) = run_cli().await {
        log::error!("فشل في التنفيذ: {}", e);
        process::exit(1);
    }
    
    Ok(())
}

/// عرض شعار النظام
fn print_banner() {
    println!();
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║                                                          ║");
    println!("║  ███╗   ███╗ █████╗ ██████╗ ██╗    ██╗ █████╗ ███╗   ██╗ ║");
    println!("║  ████╗ ████║██╔══██╗██╔══██╗██║    ██║██╔══██╗████╗  ██║ ║");
    println!("║  ██╔████╔██║███████║██████╔╝██║ █╗ ██║███████║██╔██╗ ██║ ║");
    println!("║  ██║╚██╔╝██║██╔══██║██╔══██╗██║███╗██║██╔══██║██║╚██╗██║ ║");
    println!("║  ██║ ╚═╝ ██║██║  ██║██║  ██║╚███╔███╔╝██║  ██║██║ ╚████║ ║");
    println!("║  ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝ ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝  ╚═══╝ ║");
    println!("║                                                          ║");
    println!("║            H U B   F A C T O R I E S   v3.0.0            ║");
    println!("║                                                          ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();
    println!("📅 التاريخ: {}", chrono::Local::now().format("%Y-%m-%d"));
    println!("⏰ الوقت: {}", chrono::Local::now().format("%H:%M:%S"));
    println!("🚀 الإصدار: {}", env!("CARGO_PKG_VERSION"));
    println!("🏭 إدارة المصانع الذكية للتعليم والإبداع");
    println!();
}
