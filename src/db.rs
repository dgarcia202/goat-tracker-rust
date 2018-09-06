use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    println!("Connecting to DB: {}", database_url);

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}