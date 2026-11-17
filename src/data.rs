use std::fs;
use std::io;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize logging (prints info, warn, and error logs to console)
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // 2. Save data to MySQL
    let username = "Alice";
    let email = "alice@example.com";

    info!("Attempting to save user: {}", username);

    let result = sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind(username)
        .bind(email)
        .execute(&pool)
        .await;

    // 3. Log success or failure
    match result {
        Ok(res) => {
            info!(
                "Saved user '{}' successfully! New ID: {}, Rows affected: {}",
                username,
                res.last_insert_id(),
                res.rows_affected()
            );
        }
        Err(e) => {
            error!("Failed to save user '{}': {}", username, e);
        }
    }

    Ok(())
}

// read file
pub fn read_file(path: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

// total line
pub fn count_lines(content: &str) -> usize {
    content.lines().count()
}

// key word match
pub fn count_matches(content: &str, keyword: &str) -> usize {
    content.matches(keyword).count()
}
