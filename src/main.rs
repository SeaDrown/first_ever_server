use axum::{
    Router,
    extract::{Path, Query},
    routing::get,
};
use std::collections::HashMap;

async fn root() -> &'static str {
    "THIS IS THE ROOT! BACK END WEB DEV HERE I COME!!"
}

#[tokio::main]
async fn main() {
    let app: Router = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/tasks", get(list_tasks).post(create_task)) // in this case, this path supports both GET and POST operation
        .route("/tasks/{id}", get(get_task_from_id)) // {id} will be replaced by the id of the requested task, /1, /999, etc
    ;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await.unwrap();

    println!("server starting...");

    axum::serve(listener, app).await.unwrap(); // this future will never finish, until the app is shut down
}

// id path is passed into the function by default, axum is wonderful!
async fn get_task_from_id(Path(id): Path<u32>) -> String {
    format!("You requested task with task ID {}", id)
}

async fn health() -> &'static str {
    "O.K"
}

async fn list_tasks() -> &'static str {
    "task list goes here"
}

async fn create_task() -> &'static str {
    "created a task!!!"
}
