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

// file size
pub fn get_file_size(path: &str) -> Result<u64, io::Error> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

// file name
pub fn get_file_name(path: &str) -> Option<String> {
    Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .map(String::from)
}

// file extension
pub fn get_file_extension(path: &str) -> Option<String> {
    Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(String::from)
}

// file information struct
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub extension: Option<String>,
    pub size: u64,
    pub total_lines: usize,
}

// get complte file information
pub fn get_file_info(path: &str) -> Result<FileInfo, io::Error> {
    let content = read_file(path)?;

    let name = get_file_name(path)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file name"))?;

    let extension = get_file_extension(path);
    let size = get_file_size(path)?;
    let total_lines = count_lines(&content);

    Ok(FileInfo {
        name,
        path: path.to_string(),
        extension,
        size,
        total_lines,
    })
}
