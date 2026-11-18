
use super::*;
use std::fs;
use sqlx::{MySqlPool, Row};
use sqlx::mysql::MySqlPoolOptions;
use std::env;

#[test]
fn read_file() {
    assert!(data::read_file("test.txt").is_ok())
}

// #[test]
// fn get_file_info() {
//     if let Ok(info) = data::get_file_info("test.txt") {
//         assert_eq!(info.name, String::from("test.txt"));
//         assert_eq!(info.path, String::from("test.txt"));
//         assert_eq!(info.extension, Some(String::from("txt")));
//         assert_eq!(info.size, 367);
//         assert_eq!(info.total_lines, 42);
//     }
//     assert!(data::get_file_info("NONE.rs").is_err());
// }

async fn create_table(pool: &MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE file_records (
            id          BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
            file_name   VARCHAR(255)  NOT NULL,
            file_path   VARCHAR(1024) NOT NULL,
            extension   VARCHAR(32)   NOT NULL,
            file_size   BIGINT UNSIGNED NOT NULL,
            total_lines BIGINT UNSIGNED NOT NULL
        )",
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[sqlx::test]
async fn connect_db(){
    dotenvy::dotenv().ok();
    assert!(env::var("DATABASE_URL").is_ok());
    let database_url = env::var("DATABASE_URL").unwrap();
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url.as_str())
        .await
        .unwrap();
}
#[sqlx::test]
async fn save_file_info(pool:MySqlPool)->sqlx::Result<()>{
    let info:data::FileInfo=data::FileInfo { 
        name: String::from("test.txt"), 
        path: String::from("test.txt"), 
        extension: Some(String::from("txt")), 
        size: (367), 
        total_lines: (42) 
    };
    create_table(&pool).await?;
    let id = data::save_file_info(&pool, &info).await?;
    assert_ne!(id,0);

    let row = sqlx::query(
        "SELECT file_name, file_path, extension, file_size, total_lines
            FROM file_records WHERE id = ?",
    )
    .bind(id)
    .fetch_one(&pool)
    .await?;

    assert_eq!(row.get::<String, _>("file_name"), "test.txt");
    assert_eq!(row.get::<String, _>("file_path"), "test.txt");
    assert_eq!(row.get::<String, _>("extension"), "txt");
    assert_eq!(row.get::<u64, _>("file_size"), 367);
    assert_eq!(row.get::<u64, _>("total_lines"), 42);
    Ok(())
}
#[sqlx::test]
async fn save_file_info_fail(pool: MySqlPool) {
    let info:data::FileInfo=data::FileInfo { 
        name: String::from("test.txt"), 
        path: String::from("test.txt"), 
        extension: Some(String::from("txt")), 
        size: (367), 
        total_lines: (42) 
    };
    let result = data::save_file_info(&pool, &info).await;
    assert!(result.is_err());
}

#[sqlx::test]
async fn save_search_log(pool:MySqlPool)->sqlx::Result<()>{
    create_table(&pool).await?;
    let info:data::FileInfo=data::FileInfo { 
        name: String::from("test.txt"), 
        path: String::from("test.txt"), 
        extension: Some(String::from("txt")), 
        size: (367), 
        total_lines: (42) 
    };
    let id = data::save_file_info(&pool, &info).await?;
    assert!(data::save_search_log(&pool, id, "test", 3).await.is_ok());

    let row=sqlx::query("SELECT file_id, keyword, match_count
        FROM search_logs WHERE id = ?",)
        .bind(id)
        .fetch_one(&pool)
        .await?;
    assert_eq!(row.get::<u64, _>("file_id"), id);
    assert_eq!(row.get::<String, _>("keyword"), "test");
    assert_eq!(row.get::<u64, _>("match_count"), 3);
    Ok(())
}