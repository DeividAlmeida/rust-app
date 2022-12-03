use mongodb::{Client, options::{ClientOptions, ResolverConfig}};

pub async fn mongodb() -> Result<Client, mongodb::error::Error> {
  let client_uri : &str = "mongodb://localhost:27017";
  let options:ClientOptions =  ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await.unwrap();
  Client::with_options(options)
}
