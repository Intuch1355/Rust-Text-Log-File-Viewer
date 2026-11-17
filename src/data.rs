use std::fs;
use std::io;

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
