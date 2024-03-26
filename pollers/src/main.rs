use std::pin::pin;
use futures::TryStreamExt;
use std::time::Duration;
use sqlx::postgres::{PgListener};
use sqlx::{Error, Executor, PgPool};
use tokio_postgres::types::ToSql;

const LISTEN_DURATION: Duration = Duration::from_secs(5);

#[tokio::main]
async fn main() -> Result<(), Error> {
    let db_url = "postgres://yellowhatpro@localhost:5432";
    let pool = PgPool::connect(&db_url).await?;
    let mut listener = PgListener::connect_with(&pool).await?;
    let notify_pool = pool.clone();
    //another thread
    let _t = tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(6));
        while !notify_pool.is_closed() {
            interval.tick().await;
            notify(&notify_pool).await;
        }
    });
    println!("Start Listen loop.");
    listener.listen_all(vec!["table_data"]).await?;
    loop {
        let mut id = 1;
        let notification = listener.recv().await.unwrap();
        println!("wow {} : {}", {id}, notification.payload());
        id=id+1;
    }



    // let mut stream = listener.into_stream();
    // let mut timeout = pin!(tokio::time::sleep(LISTEN_DURATION));
    // loop {
    //     tokio::select! {
    //         res = stream.try_next() => {
    //             if let Some(notification)  = res? {
    //                 println!("[from stream]: {notification:?}");
    //             } else {
    //                 break;
    //             }
    //         },
    //         _ = timeout.as_mut() => {
    //             break;
    //         }
    //     }
    // }
    Ok(())
}
async fn notify(pool: &PgPool){
    let res = sqlx::query(
        r#"
DO $$ 
DECLARE 
    rec RECORD;
BEGIN
    FOR rec IN SELECT * FROM internet_archive_queue
    LOOP
        PERFORM pg_notify('table_data', row_to_json(rec)::text);
    END LOOP;
END $$;            "#
    )
        .execute(pool)
        .await;
        println!("[from notify]: {res:?}");
}