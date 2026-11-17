mod data;
mod ui;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("usage: cargo run -- <file_path> <keyword>");
        return;
    }

    let path = &args[1];
    let keyword = &args[2];

    match data::read_file(path) {
        Ok(content) => {
            let lines = data::count_lines(&content);
            let matches = data::count_matches(&content, keyword);
            let matched_lines = data::filter_lines(&content, keyword);
            let size = data::get_file_size(path);
            let extension = data::get_file_extension(path);

            println!("========== LOG VIEWER ==========");
            println!("File: {}", path);

            match extension {
                Some(ext) => println!("Extension: .{}", ext),
                None => println!("Extension: No extension"),
            }

            println!("Keyword: {}", keyword);
            println!("Total lines: {}", lines);
            println!("Matches: {}", matches);

            match size {
                Ok(size) => println!("File size: {} bytes", size),
                Err(error) => println!("File size error: {}", error),
            }

            println!("\n--- ALL LINES ---");
            for (idx, line) in content.lines().enumerate() {
                println!("{}: {}", idx + 1, line);
            }

            println!("\n--- FILTERED LINES ---");
            for (line_num, line) in matched_lines {
                println!("Line {}: {}", line_num, line);
            }
            println!("================================");

            // Save file info and search log to MySQL
            if let Ok(info) = data::get_file_info(path) {
                match data::connect_db().await {
                    Ok(pool) => match data::save_file_info(&pool, &info).await {
                        Ok(file_id) => {
                            if let Err(e) =
                                data::save_search_log(&pool, file_id, keyword, matches).await
                            {
                                eprintln!("Failed to save search log to MySQL: {}", e);
                            } else {
                                println!("Saved file record (ID: {}) and search log to MySQL.", file_id);
                            }
                        }
                        Err(e) => eprintln!("Failed to save file info to MySQL: {}", e),
                    },
                    Err(e) => eprintln!("MySQL connection error: {}", e),
                }
            }
        }

        Err(error) => println!("Error: {}", error),
    }
}

#[cfg(test)]
mod unit_test;