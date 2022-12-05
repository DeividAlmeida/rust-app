use bson::doc;
use futures::{ TryStreamExt};
use mongodb::{bson::Document, Client, Collection, Cursor, options::FindOptions};
use std::{error::Error, time::Duration};
use rand::Rng;

mod connection; 

pub async fn read()-> Result<(), Box<dyn Error>>  {
  let client:Client =  connection::mongodb().await?;
  let options: FindOptions = FindOptions::builder()
  .limit(3)
  .sort(doc! { "amount": 1 })
  .build();

  let coll:Collection<Document> =  client.database("designations").collection::<Document>("publishers");
  let cursor:Cursor<Document> = coll.find(doc! {"type":{"$gt":1}}, options).await?;
 
  let mut rng = rand::thread_rng();

  let results: Vec<Document> = cursor.try_collect().await.unwrap();
  println!("{:?}", results[rng.gen_range(0..3)]);
  Ok(())
}

pub async fn create_publisher(designation: Document)-> Result<(), Box<dyn Error>>  {
  let client:Client =  connection::mongodb().await?;
  let collection:Collection<Document> = client.database("designations").collection("publishers");
  let handle=  tokio::task::spawn(async move {
    collection.insert_one(designation, None).await
  });
  tokio::time::timeout(Duration::from_secs(5), handle).await???;
  Ok(())
}
 