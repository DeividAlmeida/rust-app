use mongodb::{bson::Document, Client, options::{ClientOptions, ResolverConfig}};
use std::error::Error;
use futures::stream::TryStreamExt;

pub async fn mongodb()-> Result<(), Box<dyn Error>> {
  let client_uri : &str = "mongodb://localhost:27017";
  
  let options =  ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;
  let client  = Client::with_options(options)?;
  let coll =  client.database("local").collection::<Document>("messages");
  let mut cursor = coll.find(None, None).await?;
  while let  Some(doc) = cursor.try_next().await?  {
    println!("{}", doc);
  }
  Ok(())
}