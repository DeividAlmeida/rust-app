use chrono::Utc;
use tokio;
use bson::doc;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
mod services;

#[tokio::main]
async fn main() {
  let results = services::read().await.unwrap();
  println!("{:?}", results);
  let listener = TcpListener::bind("127.0.0.1:5252").unwrap();
  for stream in listener.incoming(){
    let stream= stream.unwrap();
    handle_connection(stream, results);
  }
  
  let who: u8 =0;
  if who == 0 {
    
  } else {
    services::create_publisher(doc! {
      "name":"Alguem",
      "type":3,
      "amount":0,
      "gender":"f",
      "active":true,
      "created_at": Utc::now(),
      "updated_at": Utc::now(),
    }).await.unwrap();
  }
}

fn handle_connection(mut stream: TcpStream,results:()){
  let mut buffer = [0;1024];
  stream.read(&mut buffer).unwrap();
  print!(
    "Resquest: {}",
    String::from_utf8_lossy(&buffer[..])
  );

  let response =   format!("HTTP/1.1 200 OK\r\n\r\n\r\n{:?}", results);
  stream.write(response.as_bytes()).unwrap();
}

//https://github.com/mehmetsefabalik/rust-mongodb-example
//https://doc.rust-lang.org/book/ch05-03-method-syntax.html


//https://practice.rs/lifetime/static.html preciso ler amanhã, aprender tbm sobre usize e isize

// refatorar codigo 