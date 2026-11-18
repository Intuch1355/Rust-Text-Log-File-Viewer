use sqlx::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

// connect to mysql using DATABASE_URL from .env
pub async fn connect_db() -> Result<MySqlPool, String> {
    // Keep an already-exported DATABASE_URL usable even if .env is absent.
    let _ = dotenvy::dotenv();
    let database_url = env::var("DATABASE_URL")
        .map_err(|error| format!("DATABASE_URL is missing or invalid: {}", error))?;

    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|error| format!("MySQL connection failed: {}", error))
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

// save FileInfo to mysql and return the new row ID
pub async fn save_file_info(pool: &MySqlPool, info: &FileInfo) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO file_records (file_name, file_path, extension, file_size, total_lines) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&info.name)
    .bind(&info.path)
    .bind(&info.extension)
    .bind(info.size)
    .bind(info.total_lines as u64)
    .execute(pool)
    .await?;

    Ok(result.last_insert_id())
}

// save search log to mysql
pub async fn save_search_log(
    pool: &MySqlPool,
    file_id: u64,
    keyword: &str,
    match_count: usize,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO search_logs (file_id, keyword, match_count) VALUES (?, ?, ?)")
        .bind(file_id)
        .bind(keyword)
        .bind(match_count as u64)
        .execute(pool)
        .await?;

    Ok(())
}


// One activity row returned from MySQL.
#[derive(Debug, Clone)]
pub struct ActivityEntry {
    pub file_name: String,
    pub keyword: String,
    pub match_count: usize,
    pub searched_at: String,
}

// Load persisted activity for the GUI.
pub async fn load_activity(pool: &MySqlPool) -> Result<Vec<ActivityEntry>, sqlx::Error> {
    use sqlx::Row;

    let rows = sqlx::query(
        "SELECT f.file_name, s.keyword, s.match_count, \
         CAST(s.searched_at AS CHAR) AS searched_at \
         FROM search_logs s \
         INNER JOIN file_records f ON f.id = s.file_id \
         ORDER BY s.searched_at DESC, s.id DESC"
    )
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| {
            Ok(ActivityEntry {
                file_name: row.try_get("file_name")?,
                keyword: row.try_get("keyword")?,
                match_count: row.try_get::<u64, _>("match_count")? as usize,
                searched_at: row.try_get("searched_at")?,
            })
        })
        .collect()
}
