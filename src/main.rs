use chrono::Utc;
use tokio;
use bson::doc;
mod services;

#[tokio::main]
async fn main() {
 
  let who: u8 =1;
  if who == 1 {
    let main = services::main_publisher().await;
    let see = services::create_presentation(main).await;
    println!("{:?}", see);
  } else {
    services::create_publisher(doc! {
      "name":"Guilherme",
      "type":1,
      "amount":0,
      "gender":"m",
      "active":true,
      "created_at": Utc::now(),
      "updated_at": Utc::now(),
    }).await.unwrap();
  }
}

//https://github.com/mehmetsefabalik/rust-mongodb-example
//https://doc.rust-lang.org/book/ch05-03-method-syntax.html


//https://practice.rs/lifetime/static.html preciso ler amanhã, aprender tbm sobre usize e isize

// refatorar codigo 