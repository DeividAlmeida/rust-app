use bson::doc;
use chrono::Utc;
use futures::{ TryStreamExt};
use futures::executor::block_on;
use mongodb::{bson::Document, options::FindOptions};
use std::{error::Error};
use rand::Rng;

mod connection; 

fn raffle() -> (usize, &'static str, i32) {
  const ORDER: [i32; 2] = [-1, 1];
  const ITEMS: [&str; 3] = ["type", "name", "updated_at"];
  
  let mut rng = rand::thread_rng();
  let item_index: usize = rng.gen_range(0..ITEMS.len());
  let order_index: usize = rng.gen_range(0..ORDER.len());
  
  (item_index, ITEMS[item_index], ORDER[order_index])
}

async fn  update_publisher(participant: bson::Document) -> Result<(), Box<dyn Error>> {
  let publisher_collection = connection::designations_publishers_conn().await.unwrap();
  
  publisher_collection.update_one(
  doc! { 
    "_id": participant.get("_id").unwrap() 
  },
  doc!{ 
    "$inc": { "amount": 1 } 
  },
  None).await.unwrap();

  Ok(())
}

pub async fn create_publisher(designation: Document) -> Result<(), Box<dyn Error>> {
  let publisher_collection = connection::designations_publishers_conn().await?;
  publisher_collection.insert_one(designation, None).await?;

  Ok(())
}

async fn get_publisher (params:Option<bson::Document>, options: FindOptions) -> Vec<Document> {
  let publisher_collection = connection::designations_publishers_conn().await.unwrap();
  let publishers_found = publisher_collection.find(params, options.clone()).await.unwrap();
  
  publishers_found.try_collect().await.unwrap()
}

pub async fn create_presentation() -> Result<(), Box<dyn Error>> {
  let (index, item, order) = raffle();

  let publisher_collection = connection::designations_presentations_conn().await.unwrap();
  let options = FindOptions::builder()
  .limit(3)
  .sort(doc! { 
      "amount": 1, 
      item : order}
  )
  .build();

  let main_publisher = block_on(get_publisher( Some(doc! {
    "type":{ "$gt":1 }
  }),options.clone()));
  let second_publisher = block_on(get_publisher( Some(doc! {
    "gender":main_publisher[index].get("gender").unwrap(),
    "name":{"$ne":main_publisher[index].get("name").unwrap()},
    "type":{ "$lte":2 }
  }),options.clone()));

  publisher_collection.insert_one(doc! {
    "main": main_publisher[index].get("name").unwrap(),
    "helper":second_publisher[index].get("name").unwrap(),
      "created_at": Utc::now()
  }, None).await?;
  
    update_publisher(main_publisher[index].clone()).await.unwrap();
    update_publisher(second_publisher[index].clone()).await.unwrap();
  
  Ok(())
}