use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new().route("/todos", get(todos_index).post(todos_create));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();

    println!("listening on {} 🚀", addr);

    axum::serve(listener, app).await.unwrap();
}

async fn todos_index() -> Json<Vec<Todo>> {
    let todos = vec![Todo {
        id: Uuid::new_v4(),
        text: String::from("Sample Todo"),
        completed: false,
    }];

    Json(todos)
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    text: String,
}

async fn todos_create(Json(input): Json<CreateTodo>) -> impl IntoResponse {
    let todo = Todo {
        id: Uuid::new_v4(),
        text: input.text,
        completed: false,
    };

    (StatusCode::CREATED, Json(todo))
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: Uuid,
    text: String,
    completed: bool,
}
