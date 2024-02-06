use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use std::env;

// File: connectionpool.rs
pub fn connect_db() -> PgConnection {
    // Load environment variables
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
