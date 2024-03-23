mod router;
mod data;
mod handlers;
mod ratelimit;

use std::sync::Arc;
use sqlx::{postgres::PgPoolOptions};
use crate::data::AppState;
use crate::router::create_router;

#[tokio::main]
async fn main() {
    let database_url = "postgres://yellowhatpro@localhost:5432";
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await {
        Ok(pool) => {
            println!("Connection to db successfull");
            pool
        }
        Err(err) => {
            println!("Fuck around and find out: {:?}", err);
            std::process::exit(1);
        }
    };
    let app = create_router(Arc::new(AppState {
        db: pool.clone()
    }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}