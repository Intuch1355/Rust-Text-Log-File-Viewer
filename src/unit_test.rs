use super::*;

#[test]
fn read_file() {
    assert!(data::read_file("test.txt").is_ok())
}
