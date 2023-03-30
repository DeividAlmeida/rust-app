use actix_web::{get, post, web::{ Path, self }, App, HttpResponse, HttpServer, Responder};
use bson::{doc, oid::ObjectId};
use chrono::Utc;
use mongodb::options::FindOptions;
use models::publisher::Publisher;
mod services;
mod models;

// #[tokio::main]
// async fn main() {
//     let result = services::create_presentation().await;
//     print!("{:?}",result.unwrap().inserted_id);

// }



#[get("/presentation")]
async fn get_presentations() -> impl Responder {
  
  let presentations = services::get_presentation().await;
  match presentations {
    Ok(response) => HttpResponse::Ok().json(response),
    Err(error) => HttpResponse::InternalServerError().body(error.to_string())
  }
}

#[post("/presentation")]
async fn create_presentations() -> impl Responder {
  let res = services::create_presentation().await;

  match res {
    Ok(response) => HttpResponse::Ok().json(response.inserted_id),
    Err(error) => HttpResponse::InternalServerError().body(error.to_string())
  }
}

#[get("/publisher/{id}")]
async fn get_publishers( path: Path<String>) -> impl Responder {
  let id = path.into_inner();
  let id_as_object = ObjectId::parse_str(id).unwrap();
  let options = FindOptions::builder()
  .limit(1)
  .build();
  let publisher = services::get_publisher(Some(doc! {"_id": id_as_object}), options.clone()).await;

  HttpResponse::Ok().json(publisher)
}

#[post("/publisher")]
async fn create_publisher(req: web::Json<Publisher>) -> impl Responder {

  let res = services::create_publisher(Publisher {
    id:None,
    name: req.name.clone(),
    gender: req.gender.clone(),
    r#type:req.r#type.clone(),
    amount:Some(0),
    active:Some(true),
    created_at:Some(Utc::now().timestamp_millis()),
    updated_at:Some(Utc::now().timestamp_millis()),
  }).await;

  match res {
    Ok(response) => HttpResponse::Ok().json(response.inserted_id),
    Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
  }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(get_presentations)
        .service(create_presentations)
        .service(get_publishers)
        .service(create_publisher)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

//https://practice.rs/lifetime/static.html preciso ler amanhã, aprender tbm sobre usize e isize

// usar o Actix para construir uma api rest e o Yew para fazer o frontend
// https://github.com/actix/examples/blob/master/databases/mongodb/src/main.rs
//https://dev.to/hackmamba/build-a-rest-api-with-rust-and-mongodb-actix-web-version-ei1