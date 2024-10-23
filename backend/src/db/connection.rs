// Database Connection Logic

use std::env;

use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn establish_connection() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}",database_url);

    return PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");
}
