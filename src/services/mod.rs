use bson::doc;
use chrono::Utc;
use futures::{ TryStreamExt};
use mongodb::{options::FindOptions, results::InsertOneResult};
use std::{error::Error};
use rand::Rng;
use crate::models::presentation::Presentation;
use crate::models::publisher::Publisher;
mod connection; 

fn raffle() -> (usize, &'static str, i32) {
  const ORDER: [i32; 2] = [-1, 1];
  const ITEMS: [&str; 3] = ["type", "name", "updated_at"];
  
  let mut rng = rand::thread_rng();
  let item_index: usize = rng.gen_range(0..ITEMS.len());
  let order_index: usize = rng.gen_range(0..ORDER.len());
  
  (item_index, ITEMS[item_index], ORDER[order_index])
}

async fn  update_publisher(participant: std::option::Option<bson::oid::ObjectId>) -> Result<(), Box<dyn Error>> {
  let publisher_collection = connection::designations_publishers_conn().await.unwrap();
  
  publisher_collection.update_one(
  doc! { 
    "_id": participant
  },
  doc!{ 
    "$inc": { "amount": 1 } 
  },
  None).await.unwrap();

  Ok(())
}

pub async fn create_publisher(designation:Publisher) -> Result<InsertOneResult, Box<dyn Error>> {
  let publisher_collection = connection::designations_publishers_conn().await?;
  let publisher = publisher_collection.insert_one(designation, None).await?;

  Ok(publisher)
}

pub async fn get_publisher (params:Option<bson::Document>, options: FindOptions) -> Vec<Publisher> {
  let publisher_collection = connection::designations_publishers_conn().await.unwrap();
  let publishers_found = publisher_collection.find(params, options.clone()).await.unwrap();
  
  publishers_found.try_collect().await.unwrap()
}

pub async fn get_presentation () -> Result<Vec<Presentation>, Box<dyn Error>>  {
  let presentation_collection = connection::designations_presentations_conn().await.unwrap();
  let presentations = presentation_collection.find(None, None).await.unwrap();
  let a = presentations.try_collect().await.unwrap();
  Ok(a)
}

pub async fn create_presentation() -> Result<InsertOneResult, Box<dyn Error>> {
  let (index, item, order) = raffle();
  
  let presentation_collection = connection::designations_presentations_conn().await.unwrap();
  let options = FindOptions::builder()
  .limit(3)
  .sort(doc! { 
    "amount": 1, 
    item : order}
  )
  .build();

  let main_publisher = get_publisher( Some(doc! {
    "type":{ "$gt":1 }
  }),options.clone()).await;


  let second_publisher = get_publisher( Some(doc! {
    "gender":main_publisher[index].gender.to_string(),
    "name":{"$ne":main_publisher[index].name.to_string()},
    "type":{ "$lte":2 }
  }),options.clone()).await;


  let result = presentation_collection.insert_one(Presentation {
    id: None,
    main: main_publisher[index].name.to_string(),
    helper:second_publisher[index].name.to_string(),
    created_at:  Utc::now().timestamp_millis(),
  }, None).await?;

  update_publisher(main_publisher[index].id).await.unwrap();
  update_publisher(second_publisher[index].id).await.unwrap();

  Ok(result)
}