mod config;
mod handler;
mod nats_client;
mod models;

use axum::{routing::post, Router};
use config::Config;
use handler::send_message_handler;
use nats_client::DaprNatsClient;
use tokio::net::TcpListener;
use tracing_subscriber;
use std::net::SocketAddr;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().init();

    let cfg = Config::from_env()?;

    tracing::info!("Loaded config: topic={}, port={}", cfg.topic, cfg.server_port);

    let nats_client = DaprNatsClient::new(&cfg.pubsub_component, &cfg.topic).await?;

    let app = Router::new()
        .route("/send", post(send_message_handler))
        .with_state(nats_client);

    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.server_port));
    tracing::info!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}