use chrono::Utc;
use tokio;
use bson::doc;
mod services;

#[tokio::main]
async fn main() {
  let who: u8 =0;
  if who == 0 {
    services::read().await.unwrap();
  } else {
    services::create_publisher(doc! {
      "name":"Madalena",
      "type":3,
      "amount":0,
      "active":true,
      "created_at": Utc::now(),
      "updated_at": Utc::now(),
    }).await.unwrap();
  }
}
//https://github.com/mehmetsefabalik/rust-mongodb-example
//https://doc.rust-lang.org/book/ch05-03-method-syntax.html
