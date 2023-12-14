use advent2023::advent_utils::get_lines_from_filepath;
use itertools::Itertools;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct ConditionRecord {
    row: String,
    contiguous_groups: Vec<i128>,
}

impl ConditionRecord {
    pub fn from_row(row: String) -> ConditionRecord {
        let row_split: Vec<&str> = row.split(" ").collect();
        let row = row_split[0].to_string();
        let contiguous_string: String = row_split[1].to_string();
        let contiguous_groups: Vec<i128> = contiguous_string
            .split(",")
            .map(|s| s.parse::<i128>().unwrap())
            .collect();
        ConditionRecord {
            row,
            contiguous_groups,
        }
    }
}

impl ConditionRecord {
    pub fn unfold_from_row(row: String) -> ConditionRecord {
        let row_split: Vec<&str> = row.split(" ").collect();
        let row = row_split[0].to_string();
        let row_concatenated = row.repeat(5);
        let contiguous_string: String = row_split[1].to_string();
        let contiguous_groups: Vec<i128> = contiguous_string
            .split(",")
            .map(|s| s.parse::<i128>().unwrap())
            .collect();
        let contiguous_groups_repeated = contiguous_groups.repeat(5);
        ConditionRecord {
            row: row_concatenated,
            contiguous_groups: contiguous_groups_repeated,
        }
    }
}

impl ConditionRecord {
    pub fn get_unknown_indices(&self) -> Vec<usize> {
        let mut unknown_indices: Vec<usize> = vec![];
        for (i, c) in self.row.chars().enumerate() {
            if c == '?' {
                unknown_indices.push(i);
            }
        }
        unknown_indices
    }
}

fn get_contiguous_springs(row: &str) -> Vec<i128> {
    let mut contiguous_springs: Vec<i128> = vec![];
    let mut current_contiguous_springs = 0;
    for c in row.chars() {
        if c == '#' {
            current_contiguous_springs += 1;
        } else {
            if current_contiguous_springs > 0 {
                contiguous_springs.push(current_contiguous_springs);
                current_contiguous_springs = 0;
            }
        }
    }
    if current_contiguous_springs > 0 {
        contiguous_springs.push(current_contiguous_springs);
    }
    contiguous_springs
}

impl ConditionRecord {
    pub fn get_n_arrangements(&self) -> i128 {
        let mut arrangements = 0;
        let n_broken_springs = self.row.chars().filter(|c| *c == '#').count();
        let expected_broken_springs: i128 = self.contiguous_groups.iter().sum();
        let fill_n: usize = expected_broken_springs as usize - n_broken_springs;
        let unknown_indices = self.get_unknown_indices();
        let combinations = unknown_indices.iter().combinations(fill_n);
        for combination in combinations {
            let mut row = self.row.clone();
            for i in &combination {
                row.replace_range(**i..**i + 1, "#");
            }
            let contiguous_springs = get_contiguous_springs(&row);
            if contiguous_springs == self.contiguous_groups {
                arrangements += 1;
            }
        }
        arrangements
    }
}

fn part_one(lines: &[String]) -> i128 {
    let mut arrangement_sum: i128 = 0;
    for line in lines {
        let condition_record = ConditionRecord::from_row(line.to_string());
        let arrangements = condition_record.get_n_arrangements();
        arrangement_sum += arrangements;
    }
    arrangement_sum
}

fn part_two(lines: &[String]) -> i128 {
    let sums: Vec<i128> = lines
        .par_iter()
        .map(|line| {
            let condition_record = ConditionRecord::unfold_from_row(line.to_string());
            let arrangements = condition_record.get_n_arrangements();
            arrangements
        })
        .collect();
    let sums_sum: i128 = sums.iter().sum();
    sums_sum
}

fn main() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_twelve_input.txt");
    let result = part_one(&lines);
    println!("{}", result);
    let result_2 = part_two(&lines);
    println!("{}", result_2);
}

#[test]
fn test_part_one() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_twelve_test.txt");
    let result = part_one(&lines);
    assert_eq!(result, 21);
}

#[test]
fn test_part_two() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_twelve_test.txt");
    let result = part_two(&lines);
    assert_eq!(result, 525152);
}

#[test]
fn test_condition_record_from_row() {
    let row = "???.### 1,1,3".to_string();
    let condition_record = ConditionRecord::from_row(row);
    assert_eq!(condition_record.row, "???.###");
    assert_eq!(condition_record.contiguous_groups, vec![1, 1, 3]);
}

#[test]
fn test_get_contigous_springs() {
    let row = "???.###.###.#";
    let contiguous_springs = get_contiguous_springs(row);
    assert_eq!(contiguous_springs, vec![3, 3, 1]);
}

//.??..??...?##. 1,1,3
#[test]
fn test_get_arrangements() {
    let row = ".??..??...?##. 1,1,3".to_string();
    let condition_record = ConditionRecord::from_row(row);
    let arrangements = condition_record.get_n_arrangements();
    assert_eq!(arrangements, 4);
}

#[test]
fn test_get_unknown_indices() {
    let row = ".??..??...?##. 1,1,3".to_string();
    let condition_record = ConditionRecord::from_row(row);
    let unknown_indices = condition_record.get_unknown_indices();
    assert_eq!(unknown_indices, vec![1, 2, 5, 6, 10]);
}
