use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    let addr = listener.local_addr().unwrap();

    println!("listening on {} 🚀", addr);

    axum::serve(listener, app).await.unwrap();
}
