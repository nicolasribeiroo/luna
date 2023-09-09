pub mod config;
pub mod modules;

use std::time::Duration;

use envconfig::Envconfig;
use tokio::time;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::{fmt, EnvFilter, Registry};

use crate::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
    let fmt_layer = fmt::layer().with_target(false);
    let subscriber = Registry::default().with(env_filter).with(fmt_layer);
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to initalize global tracing subscriber");

    let config = Config::init_from_env().unwrap();

    let mut interval = time::interval(Duration::from_secs(config.interval));

    loop {
        interval.tick().await;
        tokio::spawn(modules::lastfm::spam_single_track());
    }
}
