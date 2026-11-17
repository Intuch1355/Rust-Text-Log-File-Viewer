use super::*;
use std::fs;
#[test]
fn read_file() {
    assert!(data::read_file("test.txt").is_ok())
}
#[test]
fn filter_lines() {
    if let Ok(test) = fs::read_to_string("test.txt") {
        let test = test.as_str();
        assert_eq!(data::filter_lines(test, "Blue").len(), 6);
        assert_eq!(data::filter_lines(test, "Red").len(), 8);
        assert_eq!(data::filter_lines(test, "z").len(), 0);
    }
}
#[test]
fn get_file_info() {
    if let Ok(info) = data::get_file_info("test.txt") {
        assert_eq!(info.name, String::from("test.txt"));
        assert_eq!(info.path, String::from("test.txt"));
        assert_eq!(info.extension, Some(String::from("txt")));
        assert_eq!(info.size, 367);
        assert_eq!(info.total_lines, 42);
    }
}
