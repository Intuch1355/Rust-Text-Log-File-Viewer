use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

// connect to MySQL using DATABASE_URL from .env
pub async fn connect_db() -> Result<MySqlPool, sqlx::Error> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
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

// filter lines by keyword
pub fn filter_lines<'a>(content: &'a str, keyword: &str) -> Vec<(usize, &'a str)> {
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(keyword))
        .map(|(idx, line)| (idx + 1, line))
        .collect()
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

// get complete file information
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

// save FileInfo to MySQL and return the new row ID
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

// save search log to MySQL
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