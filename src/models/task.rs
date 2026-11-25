use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
     pub id: i32,
     pub title: String,
     pub description: String,
     pub completed: bool,
 }

 #[derive(Debug, Clone, Serialize, Deserialize)]
 pub struct CreateTaskPayload {
     pub title: String,
     pub description: String,
 }