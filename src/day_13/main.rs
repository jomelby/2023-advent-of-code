use advent2023::advent_utils::get_lines_from_filepath;

fn get_line_blocks(lines: &[String]) -> Vec<Vec<String>> {
    let mut line_blocks: Vec<Vec<String>> = Vec::new();
    let mut current_line_block: Vec<String> = Vec::new();
    for line in lines.iter() {
        if line == "" {
            line_blocks.push(current_line_block);
            current_line_block = Vec::new();
        } else {
            current_line_block.push(line.to_string());
        }
    }
    line_blocks.push(current_line_block);
    line_blocks
}

fn check_line_block_horizontally(line_block: &[String]) -> i128 {
    let mut horizontal_sum: i128 = 0;
    let mut potential_symmetry_points: Vec<usize> = Vec::new();
    for i in 0..line_block.len() - 1 {
        let line_1 = &line_block[i];
        let line_2 = &line_block[i + 1];
        if line_1 == line_2 {
            potential_symmetry_points.push(i);
        }
    }
    for potential in potential_symmetry_points.iter() {
        let mut symmetry_point = true;
        let range_to_check: Vec<usize> = (0..*potential).rev().collect();
        for (i, j) in range_to_check.iter().enumerate() {
            let left_index = j;
            let right_index = potential + i + 2;
            if right_index > line_block.len() - 1 {
                break;
            } else if !symmetry_point {
                break;
            } else {
                if line_block[*left_index] != line_block[right_index] {
                    symmetry_point = false;
                }
            }
        }
        if symmetry_point {
            horizontal_sum += 100 * (*potential as i128 + 1);
            return horizontal_sum;
        }
    }
    horizontal_sum
}

fn get_line_block_horizontally(line_block: &[String]) -> Vec<i128> {
    let mut horizontal_values: Vec<i128> = Vec::new();
    let mut potential_symmetry_points: Vec<usize> = Vec::new();
    for i in 0..line_block.len() - 1 {
        let line_1 = &line_block[i];
        let line_2 = &line_block[i + 1];
        if line_1 == line_2 {
            potential_symmetry_points.push(i);
        }
    }
    for potential in potential_symmetry_points.iter() {
        let mut symmetry_point = true;
        let range_to_check: Vec<usize> = (0..*potential).rev().collect();
        for (i, j) in range_to_check.iter().enumerate() {
            let left_index = j;
            let right_index = potential + i + 2;
            if right_index > line_block.len() - 1 {
                break;
            } else if !symmetry_point {
                break;
            } else {
                if line_block[*left_index] != line_block[right_index] {
                    symmetry_point = false;
                }
            }
        }
        if symmetry_point {
            horizontal_values.push(100 * (*potential as i128 + 1));
        }
    }
    horizontal_values
}

fn check_line_block_vertically(line_block: &[String]) -> i128 {
    let mut vertical_sum: i128 = 0;
    let mut potential_symmetry_points: Vec<usize> = Vec::new();
    for i in 0..line_block[0].len() - 1 {
        let mut columns_equal = true;
        for j in 0..line_block.len() {
            if line_block[j].chars().nth(i) != line_block[j].chars().nth(i + 1) {
                columns_equal = false;
                break;
            }
        }
        if columns_equal {
            potential_symmetry_points.push(i);
        }
    }
    for potential_symmetry_point in potential_symmetry_points.iter() {
        let mut symmetry_point = true;
        let range_to_check: Vec<usize> = (0..*potential_symmetry_point).rev().collect();
        for (i, j) in range_to_check.iter().enumerate() {
            let left_index = j;
            let right_index = potential_symmetry_point + i + 2;
            if right_index > line_block[0].len() - 1 {
                break;
            } else if !symmetry_point {
                break;
            } else {
                for k in 0..line_block.len() {
                    if line_block[k].chars().nth(*left_index)
                        != line_block[k].chars().nth(right_index)
                    {
                        symmetry_point = false;
                        break;
                    }
                }
            }
        }
        if symmetry_point {
            vertical_sum += *potential_symmetry_point as i128 + 1;
        }
    }
    vertical_sum
}

fn get_check_line_block_vertically(line_block: &[String]) -> Vec<i128> {
    let mut vertical_sums: Vec<i128> = Vec::new();
    let mut potential_symmetry_points: Vec<usize> = Vec::new();
    for i in 0..line_block[0].len() - 1 {
        let mut columns_equal = true;
        for j in 0..line_block.len() {
            if line_block[j].chars().nth(i) != line_block[j].chars().nth(i + 1) {
                columns_equal = false;
                break;
            }
        }
        if columns_equal {
            potential_symmetry_points.push(i);
        }
    }
    for potential_symmetry_point in potential_symmetry_points.iter() {
        let mut symmetry_point = true;
        let range_to_check: Vec<usize> = (0..*potential_symmetry_point).rev().collect();
        for (i, j) in range_to_check.iter().enumerate() {
            let left_index = j;
            let right_index = potential_symmetry_point + i + 2;
            if right_index > line_block[0].len() - 1 {
                break;
            } else if !symmetry_point {
                break;
            } else {
                for k in 0..line_block.len() {
                    if line_block[k].chars().nth(*left_index)
                        != line_block[k].chars().nth(right_index)
                    {
                        symmetry_point = false;
                        break;
                    }
                }
            }
        }
        if symmetry_point {
            vertical_sums.push(*potential_symmetry_point as i128 + 1);
        }
    }
    vertical_sums
}

fn part_one(lines: &[String]) -> i128 {
    let mut answer: i128 = 0;
    for line_block in get_line_blocks(lines).iter() {
        let horizontal_answer = check_line_block_horizontally(&line_block);
        let vertical_answer = check_line_block_vertically(&line_block);
        if horizontal_answer != 0 {
            answer += horizontal_answer;
        } else if vertical_answer != 0 {
            answer += vertical_answer;
        } else {
            // println!("{}", line_block.join("\n"));
        }
    }
    answer
}

fn part_two(lines: &[String]) -> i128 {
    let mut answer: i128 = 0;
    for line_block in get_line_blocks(lines).iter() {
        let original_horizontal_answer = check_line_block_horizontally(&line_block);
        let original_vertical_answer = check_line_block_vertically(&line_block);
        println!("{}", original_horizontal_answer);
        let mut match_found = false;
        for y in 0..line_block.len() {
            for x in 0..line_block[0].len() {
                let mut smudge_fixed_line_block: Vec<String> = Vec::new();
                for (i, line) in line_block.iter().enumerate() {
                    if y == i {
                        let mut chars: Vec<char> = line.chars().collect();
                        if chars[x] == '#' {
                            chars[x] = '.';
                        } else {
                            chars[x] = '#';
                        }
                        smudge_fixed_line_block.push(chars.into_iter().collect())
                    } else {
                        smudge_fixed_line_block.push(line.clone());
                    }
                }
                let horizontal_answers = get_line_block_horizontally(&smudge_fixed_line_block);
                for horizontal_answer in horizontal_answers.iter() {
                    if *horizontal_answer != original_horizontal_answer {
                        // println!("{}", smudge_fixed_line_block.join("\n"));
                        println!("{}", horizontal_answer);
                        answer += horizontal_answer;
                        match_found = true;
                        break;
                    }
                }
                let vertical_answers = get_check_line_block_vertically(&smudge_fixed_line_block);
                for vertical_answer in vertical_answers.iter() {
                    if *vertical_answer != original_vertical_answer {
                        // println!("{}", smudge_fixed_line_block.join("\n"));
                        println!("{}", vertical_answer);
                        answer += vertical_answer;
                        match_found = true;
                        break;
                    }
                }
                if match_found {
                    break;
                }
            }
            if match_found {
                break;
            }
        }
        if !match_found {
            println!("{}", line_block.join("\n"));
        }
    }
    answer
}

fn main() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_thirteen_input.txt");
    let result = part_one(&lines);
    println!("{}", result);
    let result_2 = part_two(&lines);
    println!("{}", result_2);
}

#[test]
fn test_part_one() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_thirteen_test.txt");
    let result = part_one(&lines);
    assert_eq!(result, 405);
}

#[test]
fn test_part_one_extra() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_thirteen_test_2.txt");
    let result = part_one(&lines);
    assert_eq!(result, 500);
}

#[test]
fn test_part_two() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_thirteen_test.txt");
    let result = part_two(&lines);
    assert_eq!(result, 400);
}
