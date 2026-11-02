use axum_db_demo::{router, startup_message, tcp_listener};

#[tokio::main]
async fn main() {
    println!("{}", startup_message());
    axum::serve(tcp_listener().await, router()).await.unwrap();
}
