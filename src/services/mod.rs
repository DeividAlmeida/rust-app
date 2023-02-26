use bson::doc;
use futures::{ TryStreamExt};
use mongodb::{bson::Document, options::FindOptions};
use std::{error::Error};
use rand::Rng;

mod connection; 

pub async fn main_publisher()-> Vec<bson::Document> {
  let mut vec_designation = vec![];
  
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
  let frist_publisher_gender = frist_publisher_results[index].get("gender");
  let frist_publisher_name = frist_publisher_results[index].get("name");
  
  vec_designation.push(frist_publisher_results[0].clone());

  let secound_publisher_results = coll.find(doc! {
    "name":{"$ne":frist_publisher_name.clone()},
    "gender":frist_publisher_gender.clone(),
    "type":{ "$lte":2 }
  },
   options.clone()).await.unwrap();
  let secound_publisher: Vec<Document> = secound_publisher_results.try_collect().await.unwrap();

  vec_designation.push(secound_publisher[0].clone());

  vec_designation
}

fn raffle() -> (usize, &'static str, i32) {
  let mut rng = rand::thread_rng();
  const ORDER: [i32; 2] = [-1, 1];
  const ITEMS: [&str; 3] = ["type", "name", "updated_at"];
  
  let item_index: usize = rng.gen_range(0..ITEMS.len());
  let order_index: usize = rng.gen_range(0..ORDER.len());
  
  (item_index, ITEMS[item_index], ORDER[order_index])
}

async fn  count_publisher(participants: bson::Document) -> Result<(), Box<dyn Error>> {
  let client = connection::mongodb().await.unwrap();
  let coll = client.database("designations").collection::<Document>("publishers");
  
  coll.update_one(
  doc! { 
    "_id": participants.get("_id").unwrap() 
  },
  doc!{ 
    "$inc": { "amount": 1 } 
  },
  None).await.unwrap();


  Ok(())
}

pub async fn create_publisher(designation: Document) -> Result<(), Box<dyn Error>> {
  let client = connection::mongodb().await?;
  let collection = client.database("designations").collection("publishers");
  collection.insert_one(designation, None).await?;

  Ok(())
}

pub async fn create_presentation(participants: Vec<bson::Document>) -> Result<(), Box<dyn Error>> {
  let client = connection::mongodb().await?;
  let collection = client.database("designations").collection("presentations");
  
  collection.insert_one(doc! {
    "main": participants[0].get("name").unwrap(),
    "helper":participants[1].get("name").unwrap()
  }, None).await?;
  
print!("{:?}", participants);
  for i in 0..2 {
    count_publisher(participants[i].clone()).await.unwrap();
  }
  
  Ok(())
}
