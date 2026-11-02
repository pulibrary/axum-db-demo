use crate::dog::handle_dog_request;
use axum::{Router, routing::get};
use std::env;
use tokio::net::TcpListener;

mod diesel;
mod dog;

const PORT_ENV_VAR: &str = "AXUM_PORT";
const DEFAULT_PORT: &str = "3333";

/// Create a new router for our app
pub fn router() -> Router {
    Router::new().route("/diesel", get(|| async { handle_dog_request() }))
}

/// Create a new listener for traffic on the port
pub async fn tcp_listener() -> TcpListener {
    TcpListener::bind(&listen_address()).await.unwrap()
}

/// A message that tells the user what the server is doing
pub fn startup_message() -> String {
    format!("Server is listening on {}", listen_address())
}

fn listen_port() -> String {
    env::var(PORT_ENV_VAR).unwrap_or(DEFAULT_PORT.to_string())
}

fn listen_address() -> String {
    format!("0.0.0.0:{}", listen_port())
}
