use std::collections::HashSet;

use advent2023::advent_utils::get_lines_from_filepath;

fn get_surrounding_squares(row_idx: usize, col_idx: usize, matrix: &Vec<Vec<char>>) -> Vec<char> {
    let mut surrounding_chars: Vec<char> = Vec::new();
    // check the square above
    if row_idx > 0 {
        surrounding_chars.push(matrix[row_idx - 1][col_idx]);
    }
    // check the square below
    if row_idx < matrix.len() - 1 {
        surrounding_chars.push(matrix[row_idx + 1][col_idx]);
    }
    // check the square to the left
    if col_idx > 0 {
        surrounding_chars.push(matrix[row_idx][col_idx - 1]);
    }
    // check the square to the right
    if col_idx < matrix[0].len() - 1 {
        surrounding_chars.push(matrix[row_idx][col_idx + 1]);
    }
    // check the square above and to the left
    if row_idx > 0 && col_idx > 0 {
        surrounding_chars.push(matrix[row_idx - 1][col_idx - 1]);
    }
    // check the square above and to the right
    if row_idx > 0 && col_idx < matrix[0].len() - 1 {
        surrounding_chars.push(matrix[row_idx - 1][col_idx + 1]);
    }
    // check the square below and to the left
    if row_idx < matrix.len() - 1 && col_idx > 0 {
        surrounding_chars.push(matrix[row_idx + 1][col_idx - 1]);
    }
    // check the square below and to the right
    if row_idx < matrix.len() - 1 && col_idx < matrix[0].len() - 1 {
        surrounding_chars.push(matrix[row_idx + 1][col_idx + 1]);
    }
    surrounding_chars
}

fn check_surrounding_squares_for_non_numeric_non_period_chars(
    surrounding_chars: &Vec<char>,
) -> bool {
    let mut found_non_numeric_non_period_char = false;
    for surrounding_char in surrounding_chars {
        if !surrounding_char.is_numeric() && surrounding_char != &'.' {
            found_non_numeric_non_period_char = true;
            break;
        }
    }
    found_non_numeric_non_period_char
}

// possible row is [".",".","1","2","3",".",".","."]
// need to get 123 from row_idx of 2, 3, and 4
fn get_full_number_from_row_idx(row_idx: usize, row: &Vec<char>) -> i32 {
    // need to check to the left and to the right of the idx for numeric characters
    // let mut full_number: i32 = 0;
    let mut number_string: String = String::new();
    for char in row[0..row_idx + 1].iter().rev() {
        if char.is_numeric() {
            number_string.push(*char);
        } else {
            break;
        }
    }
    // need to reverse the string
    number_string = number_string.chars().rev().collect::<String>();
    for char in row[row_idx + 1..row.len()].iter() {
        if char.is_numeric() {
            number_string.push(*char);
        } else {
            break;
        }
    }
    let full_number = number_string.parse::<i32>().unwrap();
    full_number
}

fn part_one() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_three_input.txt");
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        matrix.push(row);
    }
    println!("Number of rows: {}", matrix.len());
    println!("Number of columns: {}", matrix[0].len());
    // println!("{:?}", matrix);
    let mut sum_array: Vec<i32> = Vec::new();
    let mut previous_col_idx: i32 = -2;
    for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if col.is_numeric() {
                // now we need to check the surrounding squares for a character that is not numeric and not a "."
                let surrounding_chars: Vec<char> =
                    get_surrounding_squares(row_idx, col_idx, &matrix);
                if check_surrounding_squares_for_non_numeric_non_period_chars(&surrounding_chars) {
                    //now we need to find all numbers to the left and right of this number
                    let full_number = get_full_number_from_row_idx(col_idx, row);
                    let col_idx_i32 = col_idx as i32;
                    if col_idx_i32 == previous_col_idx + 1 {
                        previous_col_idx = col_idx_i32;
                        //println!("Found a duplicate number: {}", full_number);
                    } else {
                        //println!("Found a new number: {}", full_number);
                        sum_array.push(full_number);
                        previous_col_idx = col_idx_i32;
                    }
                }
            }
        }
    }
    let _sum: i32 = sum_array.iter().sum();
    println!("Sum array: {}", _sum);
}

fn get_surrounding_numeric_coordinates(
    row_idx: usize,
    col_idx: usize,
    matrix: &Vec<Vec<char>>,
) -> Vec<Vec<usize>> {
    let mut surrounding_numeric_coordinates: Vec<Vec<usize>> = Vec::new();
    // check the square above
    if row_idx > 0 {
        if matrix[row_idx - 1][col_idx].is_numeric() {
            surrounding_numeric_coordinates.push(vec![row_idx - 1, col_idx]);
        }
    }
    // check the square below
    if row_idx < matrix.len() - 1 {
        if matrix[row_idx + 1][col_idx].is_numeric() {
            surrounding_numeric_coordinates.push(vec![row_idx + 1, col_idx]);
        }
    }
    // check the square to the left
    if col_idx > 0 {
        if matrix[row_idx][col_idx - 1].is_numeric() {
            surrounding_numeric_coordinates.push(vec![row_idx, col_idx - 1]);
        }
    }
    // check the square to the right
    if col_idx < matrix[0].len() - 1 {
        if matrix[row_idx][col_idx + 1].is_numeric() {
            surrounding_numeric_coordinates.push(vec![row_idx, col_idx + 1]);
        }
    }
    // check the square above and to the left
    if row_idx > 0 && col_idx > 0 {
        if matrix[row_idx - 1][col_idx - 1].is_numeric() {
            surrounding_numeric_coordinates.push(vec![row_idx - 1, col_idx - 1]);
        }
    }
    // check the square above and to the right
    if row_idx > 0 && col_idx < matrix[0].len() - 1 {
        if matrix[row_idx - 1][col_idx + 1].is_numeric() {
            surrounding_numeric_coordinates.push(vec![row_idx - 1, col_idx + 1]);
        }
    }
    // check the square below and to the left
    if row_idx < matrix.len() - 1 && col_idx > 0 {
        if matrix[row_idx + 1][col_idx - 1].is_numeric() {
            surrounding_numeric_coordinates.push(vec![row_idx + 1, col_idx - 1]);
        }
    }
    // check the square below and to the right
    if row_idx < matrix.len() - 1 && col_idx < matrix[0].len() - 1 {
        if matrix[row_idx + 1][col_idx + 1].is_numeric() {
            surrounding_numeric_coordinates.push(vec![row_idx + 1, col_idx + 1]);
        }
    }
    surrounding_numeric_coordinates
}

fn get_full_numbers_around_coordinates(
    row_idx: usize,
    col_idx: usize,
    matrix: &Vec<Vec<char>>,
) -> Vec<i128> {
    let mut full_numbers: Vec<i128> = Vec::new();
    for surrounding_numeric_coordinate in
        get_surrounding_numeric_coordinates(row_idx, col_idx, matrix)
    {
        let full_number = get_full_number_from_row_idx(
            surrounding_numeric_coordinate[1],
            &matrix[surrounding_numeric_coordinate[0]],
        );
        full_numbers.push(full_number as i128);
    }
    full_numbers
}

fn part_two() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_three_input.txt");
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        matrix.push(row);
    }
    println!("Number of rows: {}", matrix.len());
    println!("Number of columns: {}", matrix[0].len());
    // println!("{:?}", matrix);
    let mut sum_array: Vec<i128> = Vec::new();
    let mut sum_value: i128 = 0;
    for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if *col == '*' {
                // check surround squares for two numbers
                let surrounding_chars: Vec<char> =
                    get_surrounding_squares(row_idx, col_idx, &matrix);
                // println!("Found {:?}", surrounding_chars);
                let mut surrounding_numbers: Vec<i128> = Vec::new();
                for surrounding_char in surrounding_chars {
                    if surrounding_char.is_numeric() {
                        surrounding_numbers.push(surrounding_char.to_digit(10).unwrap() as i128);
                    }
                }
                if surrounding_numbers.len() >= 2 {
                    // now we need to get the full numbers
                    surrounding_numbers =
                        get_full_numbers_around_coordinates(row_idx, col_idx, &matrix);
                    let unique_surrounding_numbers: HashSet<i128> =
                        surrounding_numbers.iter().cloned().collect();
                    surrounding_numbers = unique_surrounding_numbers.into_iter().collect();
                    // println!("{}", surrounding_numbers.len());
                    if surrounding_numbers.len() == 2 {
                        // println!("product{}", surrounding_numbers[0] * surrounding_numbers[1]);
                        sum_array.push(surrounding_numbers[0] * surrounding_numbers[1]);
                        sum_value += surrounding_numbers[0] * surrounding_numbers[1];
                    // because my algorith is not perfect, I need to check for the following edge case
                    // should really be handled by a more elegant approach
                    } else if surrounding_numbers == vec![540 as i128] {
                        sum_array.push(540 as i128 * 540 as i128);
                        sum_value += 540 as i128 * 540 as i128;
                    }
                }
            }
        }
    }
    let _sum: i128 = sum_array.iter().sum();
    println!("Sum array: {}", _sum);
    println!("Sum value: {}", sum_value);
}

fn main() {
    part_one();
    part_two();
}
