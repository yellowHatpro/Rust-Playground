use axum::response::Html;
use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
        .route("/hello",
               get(|| async { Html("Hello <strong>World!!!</strong>") }),
    );
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes_hello).await.unwrap();
}