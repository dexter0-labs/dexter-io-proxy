use tracing_subscriber::EnvFilter;
use tracing::{warn, info, error};

/// Initialize the tracing for the application
pub fn init_tracing() {
    let filter: EnvFilter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();
}