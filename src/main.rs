mod handlers;
mod models;
mod repositories;
mod services;


use axum::{
    routing::get,
    Router,
};
use crate::handlers::taskHandler;
use crate::models::task::Task;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type DbState = Arc<RwLock<Vec<Task>>>;


#[tokio::main]
async fn main() {

    let shared_db = Arc::new(RwLock::new(Vec::new()));

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/tasks", taskHandler::taskHandler())
        .with_state(shared_db);


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    
    axum::serve(listener, app).await.unwrap();
}