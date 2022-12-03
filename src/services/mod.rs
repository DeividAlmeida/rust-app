use futures::TryStreamExt;
use mongodb::{bson::Document, Client, Collection, Cursor};
use std::error::Error;

mod connection; 

pub async fn read()-> Result<(), Box<dyn Error>>  {
  let client:Client =  connection::mongodb().await.unwrap();
  let coll:Collection<Document> =  client.database("local").collection::<Document>("messages");
  let mut cursor:Cursor<Document> = coll.find(None, None).await?;
 
  while let  Some(doc) = cursor.try_next().await?  {
    println!("{}", doc);
  }
  Ok(())
}