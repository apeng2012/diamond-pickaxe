use axum::{response::IntoResponse, routing::get, Router};
use serde::Serialize;

#[derive(Serialize)]
struct Student {
    id: String,
    name: String,
    age: u8,
}

async fn json() -> impl IntoResponse {
    let students = [
        Student {
            id: "001".to_string(),
            name: "tom".to_string(),
            age: 18,
        },
        Student {
            id: "002".to_string(),
            name: "jerry".to_string(),
            age: 19,
        },
        Student {
            id: "003".to_string(),
            name: "tony".to_string(),
            age: 20,
        },
    ];

    axum::Json(students)
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/students", get(json));

    // run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
