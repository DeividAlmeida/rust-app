use chrono::Utc;
use tokio;
use bson::doc;
mod services;

#[tokio::main]
async fn main() {
 
  let who: u8 =1;
  if who == 1 {
    let see = services::create_presentation().await;
    print!("{:?}",see);
  } else {
    services::create_publisher(doc! {
      "name":"Algostinha",
      "type":1,
      "amount":0,
      "gender":"f",
      "active":true,
      "created_at": Utc::now(),
      "updated_at": Utc::now(),
    }).await.unwrap();
  }
}

//https://practice.rs/lifetime/static.html preciso ler amanhã, aprender tbm sobre usize e isize

// refatorar codigo 

// usar o Actix para construir uma api rest e o Yew para fazer o frontend