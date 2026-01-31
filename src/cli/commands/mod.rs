pub mod serve;
pub mod factory;
pub mod manufacture;
pub mod mhos;
pub mod generate;
pub mod analyze;
pub mod system;
pub mod docs;

// إعادة التصدير
pub use serve::execute as serve_execute;
pub use factory::execute as factory_execute;
pub use manufacture::execute as manufacture_execute;
pub use mhos::execute as mhos_execute;
pub use generate::execute as generate_execute;
pub use analyze::execute as analyze_execute;
pub use system::execute as system_execute;
pub use docs::execute as docs_execute;
