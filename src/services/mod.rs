use bson::doc;
use futures::{ TryStreamExt};
use mongodb::{bson::Document, Client, Collection, Cursor, options::FindOptions};
use std::{error::Error, time::Duration};
use rand::Rng;

mod connection; 

pub async fn read()-> Result<(), Box<dyn Error>>  {
  let raffled:(usize, &'static str, i32) = raffle();
  let index: usize = raffled.0;
  let item:&str = raffled.1;
  let order: i32 = raffled.2;

  let client:Client =  connection::mongodb().await?;
  let options: FindOptions = FindOptions::builder()
  .limit(3)
  .sort(doc! { 
    "amount": 1, 
    item : order}
  )
  .build();

  let coll:Collection<Document> =  client.database("designations").collection::<Document>("publishers");
  let cursor:Cursor<Document> = coll.find(doc! {"type":{"$gt":1}}, options).await?;
 

  let results: Vec<Document> = cursor.try_collect().await.unwrap();

  Ok(())
}

fn raffle() -> (usize, &'static str, i32) {
  let mut rng = rand::thread_rng();

  let orders: Vec<i32> = vec![-1,1];
  let order_index: usize = rng.gen_range(0..2);
  let order: i32 = orders[order_index];
  
  let items: Vec<&str> = vec!["_id","type","name","created_at","updated_at"];
  let item_index: usize = rng.gen_range(0..5);
  let item: &str = items[item_index];
  let results_index: usize = rng.gen_range(0..3);
 (results_index, item, order )
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
 