use dotenvy::dotenv;
use sea_orm::Database;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env

    let database_url = env::var("DATABASE_URL")?;
    let db = Database::connect(&database_url).await?;

    println!("✅ Connected to MySQL");

    Ok(())
}
