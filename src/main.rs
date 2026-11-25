mod handlers;
mod models;
mod repositories;
mod services;

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use crate::handlers::taskHandler;
use crate::models::task::Task;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type DbState = Arc<RwLock<Vec<Task>>>;

async fn global_exception_handler(req: Request, next: Next) -> Response {
    let response = next.run(req).await;
    if response.status().is_server_error() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "Internal Server Error" })),
        )
            .into_response();
    }
    response
}

async fn fallback_handler() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "error": "Route not found" })),
    )
}

async fn admin_middleware(req: Request, next: Next) -> Response {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    match token {
        Some("admin") | Some("Bearer admin") => next.run(req).await,
        _ => (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "Unauthorized" })),
        )
            .into_response(),
    }
}

async fn admin_hello() -> impl IntoResponse {
    Json(serde_json::json!({ "message": "Hello, Admin!" }))
}

#[tokio::main]
async fn main() {
    let shared_db = Arc::new(RwLock::new(Vec::new()));

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/admin/hello",
            get(admin_hello).layer(middleware::from_fn(admin_middleware)),
        )
        .nest("/tasks", taskHandler::taskHandler())
        .fallback(fallback_handler)
        .layer(middleware::from_fn(global_exception_handler))
        .with_state(shared_db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
