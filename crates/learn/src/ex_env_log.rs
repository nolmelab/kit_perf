use log::{debug, error, warn, info};

fn main() {
    let mut builder = env_logger::Builder::from_default_env();

    builder
    .filter(None, log::LevelFilter::Info)
    .init();

    debug!("Hello logger!");
    info!("Hello logger!");
    warn!("Hello logger!");
    error!("Hello logger!");
}