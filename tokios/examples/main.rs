use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    //Connection to mini-redis address
    let mut client = client::connect("127.0.0.1:8080").await?;
    // "hello" -> "world"
    client.set("hello","world".into()).await?;
    let result = client.get("hello").await?;
    println!("From Server; result: {:?}", result);
    Ok(())
}