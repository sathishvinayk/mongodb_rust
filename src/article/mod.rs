pub mod dbrepo;
use serde::{Deserialize, Serialize};
use std::io::{Error};
use mongodb::{
    Client, 
    options::{ClientOptions as opt}, 
};
use dotenv;
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    pub name: String,
    pub author: String
}

pub struct DB {
    pub client: Client,
}

// Initiate the DB
pub async fn init() -> Result<Client, Error> {
    dotenv::dotenv().ok();
    let addr = env::var("MONGO_ADDR").expect("MONGO_ADDR must be set");

    let client = opt::parse(&addr).await.unwrap();

    let connection = Client::with_options(client).unwrap();

    Ok(connection)
}