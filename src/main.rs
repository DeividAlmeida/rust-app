use tokio;
mod connection;

#[tokio::main]
async fn main() {
  connection::mongodb().await.unwrap();
}




//https://www.mongodb.com/developer/languages/rust/rust-mongodb-crud-tutorial/