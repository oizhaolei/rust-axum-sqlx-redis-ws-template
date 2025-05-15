use crate::config::Config;
use dotenv::dotenv;
use std::net::SocketAddr;
use tracing::{debug, info};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod app;
mod cache;
mod config;
mod controllers;
mod db;
mod error;
mod models;
mod repositories;
mod router;
mod services;
#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let config = Config::init();

    info!("Connecting to pg: {}", &config.database_url);
    info!("Connecting to cache: {}", &config.cache_url);

    let app = app::create_app(&config).await;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    debug!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
