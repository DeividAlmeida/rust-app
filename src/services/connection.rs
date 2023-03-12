use bson::Document;
use mongodb::{Client, options::{ClientOptions, ResolverConfig}, Collection};

pub async fn mongodb() -> Result<Client, mongodb::error::Error> {
  let client_uri : &str = "mongodb://localhost:27017";
  let options:ClientOptions =  ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await.unwrap();
  Client::with_options(options)
}

pub async fn designations_publishers_conn() -> Result<Collection<Document>, mongodb::error::Error> {
  let client = mongodb().await?;
  let collection = client.database("designations").collection("publishers");
  Ok(collection)
}

pub async fn designations_presentations_conn() -> Result<Collection<Document>, mongodb::error::Error> {
  let client = mongodb().await?;
  let collection = client.database("designations").collection("presentations");
  Ok(collection)
}