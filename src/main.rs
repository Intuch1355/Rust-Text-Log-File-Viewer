mod data;
mod ui;



fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let keyword = &args[2];

    match data::read_file(path) {
        Ok(content) => {
            let lines = data::count_lines(&content);
            let matches = data::count_matches(&content, keyword);
            println!("Total lines: {}", lines);
            println!("Matches: {}", matches);
        }
        Err(error) => println!("error: {}", error),
    }
}
#[cfg(test)]
mod unit_test;