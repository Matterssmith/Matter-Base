pub mod core;
pub mod events;
pub mod i18n;
pub mod modules;
pub mod services;

pub use core::config::CoreConfig;
pub use core::context::MatterContext;
pub use core::logger::init_logger;
pub use core::runtime::MatterRuntime;

pub use events::EventBus;
pub use i18n::Translator;
pub use modules::CoreModule;
pub use services::ServiceRegistry;