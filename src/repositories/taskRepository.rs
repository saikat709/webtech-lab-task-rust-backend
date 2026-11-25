use crate::models::task::Task;
use crate::DbState;

pub async fn getAllTasks(shared_db: &DbState) -> Vec<Task> {
    let tasks = shared_db.read().await;
    tasks.clone()
}

pub async fn createTask(title: String, description: String, shared_db: &DbState) -> Task {
    let mut tasks = shared_db.write().await;
    let id = (tasks.len() + 1) as i32;
    let task = Task { id, title, description, completed: false };
    tasks.push(task.clone());
    task
}

pub async fn updateTask(
    id: i32,
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
    shared_db: &DbState,
) -> Option<Task> {
    let mut tasks = shared_db.write().await;
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        if let Some(t) = title {
            task.title = t;
        }
        if let Some(d) = description {
            task.description = d;
        }
        if let Some(c) = completed {
            task.completed = c;
        }
        Some(task.clone())
    } else {
        None
    }
}

pub async fn deleteTask(id: i32, shared_db: &DbState) -> bool {
    let mut tasks = shared_db.write().await;
    let len_before = tasks.len();
    tasks.retain(|t| t.id != id);
    tasks.len() < len_before
}
