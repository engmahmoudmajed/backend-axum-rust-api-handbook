use axum::{
    Json, Router,
    extract::Path,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/users/{id}", get(get_user))
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, Axum! iam Mahmoud Majed join to rust backend world"
}

async fn get_user(Path(id): Path<u32>) -> Json<User> {
    Json(User {
        id,
        name: "Mahmoud".to_string(),
    })
}

async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
    Json(User {
        id: 1,
        name: payload.name,
    })
}
