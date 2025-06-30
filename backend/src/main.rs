use std::net::SocketAddr;

use axum::{routing::{get, post}, Router};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod llm;
mod session_mgmt;
mod endpoints;
mod entitiy;

// export OPENAI_API_KEY=sk-proj-XXXXXXXXXXXXXXXX

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let app = Router::new()
        .route("/api/init-session", get(endpoints::session::init_session))
        .route("/api/chat/load-history", get(endpoints::chat::fetch_history))
        .route("/api/chat/new", get(endpoints::chat::new_chat))
        .route("/api/llm/single-question", post(endpoints::chat::single_question))
        .route("/api/llm/chat-question", post(endpoints::chat::chat_question))
        .layer(CorsLayer::very_permissive());

    axum::serve(TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
