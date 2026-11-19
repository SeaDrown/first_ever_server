use crate::state::AppState;

mod handlers;
mod models;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    let state = AppState::new();

    let app = routes::app(state); // 'app' just returns the Router obj in axum

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await.unwrap(); // create a tcp listener on port 9999

    println!("server starting...");

    axum::serve(listener, app).await.unwrap(); // this future will never finish, until the app is shut down
}
