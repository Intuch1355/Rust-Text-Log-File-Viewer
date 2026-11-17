mod data;
mod ui;



fn main() {
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
            let size = data::get_file_size(path);

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

            println!("================================");
        }

        Err(error) => println!("Error: {}", error),
    }
}
#[cfg(test)]
mod unit_test;
