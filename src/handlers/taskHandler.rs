use crate::services::taskService;
use crate::DbState;
use axum::extract::Json;
use axum::{Router, routing::{
    get, patch
}};
use axum::http::StatusCode;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use crate::models::task::CreateTaskPayload;
use serde_json;

pub fn taskHandler() -> Router<DbState> {
    Router::new()
        .route("/", get(getAllTasksHandler).post(createTaskHandler))
        .route("/{id}", patch(updateTaskHandler).delete(deleteTaskHandler))
}

async fn getAllTasksHandler(State(shared_db): State<DbState>) -> impl IntoResponse {
    let tasks = taskService::getAllTasks(&shared_db).await;
    (StatusCode::OK, Json(tasks))
}

async fn createTaskHandler(
    State(shared_db): State<DbState>,
    Json(payload): Json<CreateTaskPayload>,
) -> impl IntoResponse {
    let task = taskService::createTask(payload.title, payload.description, &shared_db).await;
    (StatusCode::CREATED, Json(task))
}

async fn updateTaskHandler(
    Path(id): Path<i32>,
    State(shared_db): State<DbState>,
) -> impl IntoResponse {
    let _task = taskService::updateTask(id, None, None, None, &shared_db).await;
    (
        StatusCode::OK,
        Json(serde_json::json!({ "message": "Task updated" })),
    )
}

async fn deleteTaskHandler(
    Path(id): Path<i32>,
    State(shared_db): State<DbState>,
) -> impl IntoResponse {
    let success = taskService::deleteTask(id, &shared_db).await;
    if success {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "message": "Task not found" })),
        ))
    }
}
