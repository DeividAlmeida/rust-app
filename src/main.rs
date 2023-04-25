use chrono::Utc;
use tokio;
use bson::doc;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod services;
mod models;

#[tokio::main]
async fn main2() {
 
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

#[get("/")]
async fn get_status() -> impl Responder {
  HttpResponse::Ok().body("API is running")
}

#[get("/")]
async fn hello() -> HttpResponse {
  let presentations = services::get_presentation().await;
    HttpResponse::Ok().json(presentations)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

//https://practice.rs/lifetime/static.html preciso ler amanhã, aprender tbm sobre usize e isize

// usar o Actix para construir uma api rest e o Yew para fazer o frontend
// https://github.com/actix/examples/blob/master/databases/mongodb/src/main.rs
//https://dev.to/hackmamba/build-a-rest-api-with-rust-and-mongodb-actix-web-version-ei1