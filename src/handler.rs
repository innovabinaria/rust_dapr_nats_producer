use axum::{extract::State, http::StatusCode, Json};
use crate::models::Message;

use crate::nats_client::DaprNatsClient;


pub async fn send_message_handler(
    State(mut client): State<DaprNatsClient>,
    Json(payload): Json<Message>,
) -> Result<StatusCode, StatusCode> {

    client.publish(payload).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}