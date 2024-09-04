use std::time::Duration;

use axum::{
    error_handling::HandleErrorLayer, http::StatusCode, response::IntoResponse, routing::get, Json,
    Router,
};
use futures::executor::block_on;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Statement};
use serde::{Deserialize, Serialize};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

const DATABASE_URL: &str = "postgres://root:password@127.0.0.1:5432";
const DB_NAME: &str = "dev";

async fn run() -> Result<(), DbErr> {
    let db: DatabaseConnection =
        Database::connect("postgres://root:password@127.0.0.1:5432/dev").await?;

    let url = format!("{}/{}", DATABASE_URL, DB_NAME);
    println!("{}", url);
    Ok(())
}

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

    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }

    let app = Router::new()
        .route("/todos", get(todos_index).post(todos_create))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {} 🚀", listener.local_addr().unwrap());
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
