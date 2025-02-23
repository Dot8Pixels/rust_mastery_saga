use quests_tracker::config::config_loader;
use std::process;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let _dotenvy_env = match config_loader::load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load ENV: {}", e);
            process::exit(1);
        }
    };

    info!("ENV has been loaded");
}
