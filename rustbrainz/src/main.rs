use reqwest::StatusCode;

#[tokio::main]
async fn main(){
    println!("Hello, world!");
    get_reqwest_builder("wkw".to_string()).await;
}

async fn get_reqwest_builder(url:String) {
    let response = reqwest::get(url).await;
    match response.unwrap().status() {
        StatusCode::OK  => {} 
        StatusCode::UNAUTHORIZED => {
            
        }
        _ => {
            println!()
        }
    }
}