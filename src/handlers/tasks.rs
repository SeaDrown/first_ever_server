use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

// all of the task CRUD functionality
//
use crate::models::{CreateTask, Task, UpdateTask};
use crate::state::AppState;

// take in the app state, so we can edit the tasks list, take in create task to change taks properly
// CREATE
pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateTask>,
) -> (StatusCode, Json<Task>) {
    // lock() takes the mutex data and stops any other owner from modifying
    // unwrap() since lock() return a Result, if Err then it was already locked. in such a case, crash the app
    let mut tasks = state.tasks.lock().unwrap();

    let mut next_id = state.next_id.lock().unwrap();

    let new_task = Task {
        id: *next_id, // next_id it a reference, so must dereference
        title: payload.title,
        done: false,
    };

    *next_id += 1; // dereference again lol
    tasks.push(new_task.clone());

    (StatusCode::CREATED, Json(new_task))
}

// use the id in the path to get the task with the corresponding id
// READ
pub async fn get_from_id(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<Task>, StatusCode> {
    let tasks = state.tasks.lock().unwrap();

    tasks
        .iter()
        .find(|t| t.id == id)
        .map(|t| Json(t.clone()))
        .ok_or(StatusCode::NOT_FOUND)
}

// get an id and a Json payload of the 'UpdateTask' struct to modify an existing task
// UPDATE
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Task>, StatusCode> {
    let mut tasks = state.tasks.lock().unwrap();

    // the ? represent an early return
    let task = tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or(StatusCode::NOT_FOUND)?;

    // since UpdateTasks consists of Option<T>s, must use Some() to unwrp
    if let Some(title) = payload.title {
        task.title = title
    }
    if let Some(done) = payload.done {
        task.done = done
    }

    // Returns a Result, with Json<Task> as the type
    Ok(Json(task.clone()))
}

// just take the id from the path and delete whichever task is the same
// DESTROY
pub async fn delete(State(state): State<AppState>, Path(id): Path<u32>) -> StatusCode {
    let mut tasks = state.tasks.lock().unwrap();
    let pre_deletion_length = tasks.len();

    // retain all items, only delete the items that satisfy t.id != id
    tasks.retain(|t| t.id != id);

    // if tasks shrunk, it worked, therefore no longer exists. if not, the content was not found
    if tasks.len() < pre_deletion_length {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn list(State(state): State<AppState>) -> Json<Vec<Task>> {
    let tasks = state.tasks.lock().unwrap();

    Json(tasks.clone())
}
