use crate::repositories::taskRepository;
use crate::models::task::Task;
use crate::DbState;

pub async fn getAllTasks(shared_db: &DbState) -> Vec<Task> {
    taskRepository::getAllTasks(shared_db).await
}

pub async fn createTask(title: String, description: String, shared_db: &DbState) -> Task {
    taskRepository::createTask(title, description, shared_db).await
}

pub async fn updateTask(id: i32, title: Option<String>, description: Option<String>, completed: Option<bool>, shared_db: &DbState) -> Option<Task> {
    taskRepository::updateTask(id, title, description, completed, shared_db).await
}

pub async fn deleteTask(id: i32, shared_db: &DbState) -> bool {
    taskRepository::deleteTask(id, shared_db).await
}