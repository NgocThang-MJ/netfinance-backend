extern crate dotenv;

use dotenv::dotenv;
use mongodb::{Client, Database};
use std::env;

pub async fn connect_to_db() -> Database {
    dotenv().ok();
    let db_uri = env::var("MONGODB_URL").expect("Can't get db url");
    let client = Client::with_uri_str(db_uri)
        .await
        .expect("Failed to connect to db");

    client.database("financial-management")
}
