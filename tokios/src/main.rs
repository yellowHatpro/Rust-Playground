use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main(){
    //bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        //second item contains the IP and the port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(
            async move {
                process(socket).await;
            }
        );
    }
}

async fn process(socket: TcpStream) {
    //"Connection" -> lets us read/write redis frames instead of byte streams.
    //The "Connection" type is defined by mini-redis.
    let mut connection = Connection::new(socket);
    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("Got {:?}", frame);
        //Respond with an error
        let response = Frame::Error("TODO".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}