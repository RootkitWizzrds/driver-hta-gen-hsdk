use tracing::{info, debug};
use tracing_subscriber::FmtSubscriber;

pub fn init_logger() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set up global subscriber");

    info!("[@] Logger initialized");
}
