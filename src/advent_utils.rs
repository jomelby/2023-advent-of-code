use std::fs;

pub fn get_lines_from_filepath(filepath: &str) -> Vec<String> {
    let contents: String =
        fs::read_to_string(filepath).expect("Something went wrong reading the file");
    let mut lines: Vec<String> = Vec::new();
    for line in contents.lines() {
        lines.push(line.to_string());
    }
    lines
}
