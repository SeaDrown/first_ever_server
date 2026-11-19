use axum::{Router, routing::get};

use crate::handlers::tasks;
use crate::state::AppState;

// create the router that will direct each path to the proepr service
pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/tasks", get(tasks::list).post(tasks::create)) // in this case, this path supports both GET and POST operation
        .route(
            "/tasks/{id}",
            get(tasks::get_from_id)
                .delete(tasks::delete)
                .patch(tasks::update),
        ) // {id} will be replaced by the id of the requested task, /1, /999, etc
        .with_state(state)
}

async fn health() -> &'static str {
    "O.K"
}

async fn root() -> &'static str {
    "Home page init"
}
