use crate::events::EventBus;
use crate::services::ServiceRegistry;
use std::sync::Arc;

#[derive(Clone)]
pub struct MatterContext {
    pub events: Arc<EventBus>,
    pub services: Arc<ServiceRegistry>,
}

impl MatterContext {
    pub fn new() -> Self {
        Self {
            events: Arc::new(EventBus::new()),
            services: Arc::new(ServiceRegistry::new()),
        }
    }
}

impl Default for MatterContext {
    fn default() -> Self {
        Self::new()
    }
}