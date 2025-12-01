use std::fs;

pub fn read_to_string(path: &str) -> String {
    fs::read_to_string(path).expect("failed to read file")
}

pub fn positive_modulo(a: i32, b: i32) -> i32 {
    ((a + b) + b) % b
}

