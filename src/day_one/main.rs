use std::fs;

const NUMBER_STRINGS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_first_number(line: &str) -> char {
    let chars: Vec<char> = line.chars().collect();
    let lin_len = line.len();
    let mut first_number = '\0';
    for ix in 0..lin_len {
        if chars[ix].is_numeric() {
            first_number = chars[ix];
            break;
        }
        let truncated_line = &line[ix..lin_len];
        for (index, number_string) in NUMBER_STRINGS.iter().enumerate() {
            if truncated_line.starts_with(number_string) {
                first_number = (index + 1).to_string().chars().next().unwrap();
                break;
            }
        }
        if first_number != '\0' {
            break;
        }
    }
    first_number
}

fn find_last_number(line: &str) -> char {
    let chars: Vec<char> = line.chars().collect();
    let lin_len = line.len();
    let mut last_number = '\0';
    for ix in 1..lin_len + 1 {
        if chars[lin_len - ix].is_numeric() {
            last_number = chars[lin_len - ix];
            break;
        }
        let truncated_line = &line[lin_len - ix..lin_len];
        for (index, number_string) in NUMBER_STRINGS.iter().enumerate() {
            if truncated_line.starts_with(number_string) {
                last_number = (index + 1).to_string().chars().next().unwrap();
                break;
            }
        }
        if last_number != '\0' {
            break;
        }
    }
    last_number
}

fn main() {
    // create a dictionary to convert "one" to 1, "two" to 2, etc.

    let contents = fs::read_to_string("data/day_one_input.txt")
        .expect("Something went wrong reading the file");
    let mut sum_array: Vec<i32> = Vec::new();
    for line in contents.lines() {
        let first_number = find_first_number(line);
        let last_number = find_last_number(line);
        let line_number = format!("{}{}", first_number, last_number);
        let combined_number = line_number.parse::<i32>().unwrap();
        sum_array.push(combined_number);
    }
    let _sum: i32 = sum_array.iter().sum();
    println!("{}", _sum)
}
