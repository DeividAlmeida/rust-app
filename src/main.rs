use tokio;
mod services;

#[tokio::main]
async fn main() {
  services::read().await.unwrap();
}




//https://github.com/mehmetsefabalik/rust-mongodb-example
//https://doc.rust-lang.org/book/ch05-03-method-syntax.html
