use advent2023::advent_utils::get_lines_from_filepath;

fn get_vector_from_line(line: &str) -> Vec<i128> {
    let mut vector: Vec<i128> = Vec::new();
    for number in line.split_whitespace() {
        vector.push(number.parse::<i128>().unwrap());
    }
    vector
}

fn get_difference_vector(vector: &Vec<i128>) -> Vec<i128> {
    let mut difference_vector: Vec<i128> = Vec::new();
    for i in 0..vector.len() - 1 {
        difference_vector.push(vector[i + 1] - vector[i]);
    }
    difference_vector
}

fn check_vector_all_zeroes(vector: &Vec<i128>) -> bool {
    for number in vector.iter() {
        if *number != 0 as i128 {
            return false;
        }
    }
    true
}

fn get_next_value_from_vector(vector: &Vec<i128>) -> i128 {
    let mut difference_vectors: Vec<Vec<i128>> = Vec::new();
    let mut difference_vector: Vec<i128> = get_difference_vector(vector);
    while !check_vector_all_zeroes(&difference_vector) {
        difference_vectors.push(difference_vector.clone());
        difference_vector = get_difference_vector(&difference_vector);
    }
    let mut next_value: i128 = vector[vector.len() - 1];
    for difference_vector in difference_vectors.iter() {
        next_value += difference_vector[difference_vector.len() - 1];
    }
    next_value
}

fn get_previous_value_from_vector(vector: &Vec<i128>) -> i128 {
    let mut difference_vectors: Vec<Vec<i128>> = Vec::new();
    let mut difference_vector: Vec<i128> = get_difference_vector(vector);
    while !check_vector_all_zeroes(&difference_vector) {
        difference_vectors.push(difference_vector.clone());
        difference_vector = get_difference_vector(&difference_vector);
    }
    difference_vectors.push(difference_vector.clone());
    let mut next_value: i128 = 0;
    for difference_vector in difference_vectors.iter().rev() {
        next_value = difference_vector[0] - next_value;
    }
    next_value = vector[0] - next_value;

    next_value
}

fn part_one(lines: &[String]) -> i128 {
    let mut sum: i128 = 0;
    for line in lines.iter() {
        let vector: Vec<i128> = get_vector_from_line(line);
        sum += get_next_value_from_vector(&vector);
    }
    println!("{}", sum);
    sum
}

fn part_two(lines: &[String]) -> i128 {
    let mut sum: i128 = 0;
    for line in lines.iter() {
        let vector: Vec<i128> = get_vector_from_line(line);
        let previous_value = get_previous_value_from_vector(&vector);
        sum += previous_value;
    }
    println!("{}", sum);
    sum
}

fn main() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_nine_input.txt");
    part_one(&lines);
    part_two(&lines);
}

#[test]
fn test_part_one() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_nine_test.txt");
    assert_eq!(part_one(&lines), 114);
}

#[test]
fn test_part_two() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_nine_test.txt");
    assert_eq!(part_two(&lines), 2);
}
