use bson::doc;
use futures::{ TryStreamExt};
use mongodb::{bson::Document, options::FindOptions};
use std::{error::Error};
use rand::Rng;

mod connection; 

pub async fn main_publisher()-> bson::Document {
  let (index, item, order) = raffle();

  let client = connection::mongodb().await.unwrap();
  let options = FindOptions::builder()
      .limit(3)
      .sort(doc! { 
          "amount": 1, 
          item : order}
      )
      .build();

  let coll = client.database("designations").collection::<Document>("publishers");

  let frist_publisher_search = coll.find(doc! {"type":{ "$gt":1 }}, options.clone()).await.unwrap();
  let frist_publisher_results: Vec<Document> = frist_publisher_search.try_collect().await.unwrap();
  let frist_publisher = frist_publisher_results[index].clone();
  let frist_publisher_gender = frist_publisher.get("gender").unwrap();
  let frist_publisher_name = frist_publisher.get("name");
  
  let secound_publisher_results = coll.find(doc! {"name":{"$ne":frist_publisher_name.clone()},"type":{ "$lte":2 },"gender":{ "$eq":frist_publisher_gender.clone() }}, options.clone()).await.unwrap();
  let secound_publisher: Vec<Document> = secound_publisher_results.try_collect().await.unwrap();
  let secound_publisher_name = secound_publisher[0].get("name");

 doc! {
  "main": frist_publisher_name,
  "helper":secound_publisher_name
 }
}

fn raffle() -> (usize, &'static str, i32) {
  let mut rng = rand::thread_rng();
  const ORDER: [i32; 2] = [-1, 1];
  const ITEMS: [&str; 3] = ["type", "name", "updated_at"];
  
  let item_index: usize = rng.gen_range(0..ITEMS.len());
  let order_index: usize = rng.gen_range(0..ORDER.len());
  
  (item_index, ITEMS[item_index], ORDER[order_index])
}

pub async fn create_publisher(designation: Document) -> Result<(), Box<dyn Error>> {
  let client = connection::mongodb().await?;
  let collection = client.database("designations").collection("publishers");
  collection.insert_one(designation, None).await?;
  Ok(())
}

pub async fn create_presentation(participants: Document) -> Result<(), Box<dyn Error>> {
  let client = connection::mongodb().await?;
  let collection = client.database("designations").collection("presentations");
  collection.insert_one(participants, None).await?;
  Ok(())
}