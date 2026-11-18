use axum::{Router, routing::get};

async fn root() -> &'static str {
    "THIS IS THE ROOT! BACK END WEB DEV HERE I COME!!"
}

#[tokio::main]
async fn main() {
    let app: Router = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await.unwrap();

    println!("server starting...");

    axum::serve(listener, app).await.unwrap(); // this future will never finish, until the app is shut down
}
