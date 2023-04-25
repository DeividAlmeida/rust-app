use bson::Document;
use mongodb::{Client, options::{ClientOptions, ResolverConfig}, Collection};
use crate::models::presentation::Presentation;
use crate::models::publisher::Publisher;
use std::env;
use dotenv::dotenv;

pub async fn mongodb() -> Result<Client, mongodb::error::Error> {
  dotenv().ok();
  let client_uri  = env::var("DB_URL").expect("DB_URL not set");
  let options:ClientOptions =  ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await.unwrap();
  Client::with_options(options)
}

pub async fn designations_publishers_conn() -> Result<Collection<Document>, mongodb::error::Error> {
  let client = mongodb().await?;
  let collection = client.database("designations").collection("publishers");
  Ok(collection)
}

pub async fn designations_presentations_conn() -> Result<Collection<Presentation>, mongodb::error::Error> {
  let client = mongodb().await?;
  let collection = client.database("designations").collection("presentations");
  Ok(collection)
}