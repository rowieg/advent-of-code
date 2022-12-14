use std::fs;

pub fn read_file_to_string(fiel_path: &str) -> String {
    return String::from(fs::read_to_string(fiel_path).expect("Unable to read file"));
}
