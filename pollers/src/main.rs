use std::time::Duration;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use tokio::time::interval;

#[tokio::main]
async fn main() {
    let db_url = "postgres://yellowhatpro@localhost:5432";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();

    tokio::spawn(async move {
        database_polling_task(pool).await;
    });
    tokio::signal::ctrl_c().await.unwrap();
}

async fn database_polling_task(pool: sqlx::PgPool) {
    let mut interval = interval(Duration::from_secs(30));
    loop {
        interval.tick().await;
        if let Err(e) = poll_db(&pool).await {
            eprintln!("Error polling database: {}", e);
        }
    }
}

async fn poll_db(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    let rows = sqlx::query("SELECT * FROM test")
        .fetch_all(pool)
        .await?;
    for row in rows {
        println!("{:?}", row.columns());
    }
    Ok(())
}