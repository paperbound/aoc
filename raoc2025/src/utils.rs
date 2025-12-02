use std::fs;

pub fn read_to_string(path: &str) -> String {
    fs::read_to_string(path).expect("failed to read file")
}
