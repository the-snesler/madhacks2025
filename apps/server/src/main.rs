use std::sync::Arc;

use madhacks2025::{AppState, build_app};

const HOST: &str = "0.0.0.0";
const PORT: u16 = 3000;

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::new());
    let app = build_app(state);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", HOST, PORT))
        .await
        .unwrap();
    println!("Server running on http://{}:{}", HOST, PORT);
    axum::serve(listener, app).await.unwrap();
}
