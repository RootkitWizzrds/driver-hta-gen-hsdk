use tracing::info;
use tracing_subscriber::{FmtSubscriber, EnvFilter};

pub fn init_logger() {
    let subscriber = FmtSubscriber::builder()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set up global subscriber");

    info!("[@] Logger initialized");
}
