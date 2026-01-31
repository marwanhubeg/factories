use std::env;

fn main() {
    // تمرير إصدار Rust كمتغير بيئة
    if let Ok(rustc_version) = env::var("RUSTC_VERSION") {
        println!("cargo:rustc-env=RUSTC_VERSION={}", rustc_version);
    } else {
        // الحصول من rustc إذا لم يكن موجوداً
        let output = std::process::Command::new("rustc")
            .arg("--version")
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok());
        
        if let Some(version) = output {
            println!("cargo:rustc-env=RUSTC_VERSION={}", version.trim());
        } else {
            println!("cargo:rustc-env=RUSTC_VERSION=unknown");
        }
    }
    
    // وقت البناء
    let build_time = chrono::Utc::now().to_rfc3339();
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);
}
