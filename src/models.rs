use serde::{Deserialize, Serialize};

// this file just contains data models, which we use for CRUD on the Vec<Task>
#[derive(Serialize, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

#[derive(Deserialize, Clone)]
pub struct CreateTask {
    pub title: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub done: Option<bool>,
}
