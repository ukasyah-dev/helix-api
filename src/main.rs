use axum::{Router, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(|| async { "Hello from helix-api!" }));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Listening on http://0.0.0.0:3000");

    axum::serve(listener, router).await.unwrap();
}
