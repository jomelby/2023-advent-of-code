use std::collections::HashMap;

use advent2023::advent_utils::get_lines_from_filepath;

#[derive(Debug, Clone)]
pub struct Node {
    left: String,
    right: String,
}

fn get_key_from_line(line: &str) -> String {
    let split_line: Vec<&str> = line.split_whitespace().collect();
    let key: String = split_line[0].to_string();
    key
}

fn get_node_from_line(line: &str) -> Node {
    let mut split_line: Vec<&str> = line.split("(").collect();
    split_line = split_line[1].split(", ").collect();
    let left: String = split_line[0].to_string();
    let right: String = split_line[1].to_string().replace(")", "");
    Node { left, right }
}

fn get_hashmap_from_lines(lines: &[String]) -> HashMap<String, Node> {
    let mut hashmap: HashMap<String, Node> = HashMap::new();
    for line in lines[2..lines.len()].to_vec() {
        let key: String = get_key_from_line(&line);
        let node: Node = get_node_from_line(&line);
        hashmap.insert(key, node);
    }
    hashmap
}

fn get_starting_keys_from_hashmap(hashmap: &HashMap<String, Node>) -> Vec<String> {
    let mut starting_keys: Vec<String> = Vec::new();
    for key in hashmap.keys() {
        if key.ends_with("A") {
            starting_keys.push(key.to_string());
        }
    }
    starting_keys
}

fn get_directions_from_lines(lines: &[String]) -> Vec<char> {
    let first_line: String = lines[0].to_string();
    let directions: Vec<char> = first_line.chars().collect();
    directions
}

fn part_one(lines: &[String]) -> i64 {
    let mut step_count: i64 = 0;
    let directions: Vec<char> = get_directions_from_lines(&lines);
    let mut key = "AAA".to_string();
    let hashmap: HashMap<String, Node> = get_hashmap_from_lines(&lines);
    for direction in directions.iter().cycle() {
        let node: Node = hashmap.get(&key).unwrap().clone();
        if direction == &'L' {
            key = node.left;
        } else {
            key = node.right;
        }
        step_count += 1;
        if key == "ZZZ" {
            break;
        }
    }
    println!("step_count: {}", step_count);
    step_count
}

fn get_new_keys_from_hashmap(
    hashmap: &HashMap<String, Node>,
    keys: &Vec<String>,
    direction: &char,
) -> Vec<String> {
    let mut new_keys: Vec<String> = Vec::new();
    for key in keys {
        let node: Node = hashmap.get(key).unwrap().clone();
        if direction == &'L' {
            new_keys.push(node.left)
        } else {
            new_keys.push(node.right)
        }
    }
    new_keys
}

fn check_keys_end_with_z(keys: &Vec<String>) -> bool {
    keys.iter().all(|x| x.ends_with("Z"))
}

fn count_steps_for_key(
    key: String,
    hashmap: &HashMap<String, Node>,
    directions: &Vec<char>,
) -> i64 {
    let mut step_count: i64 = 0;
    let mut key = key.to_string();
    for direction in directions.iter().cycle() {
        let node: Node = hashmap.get(&key).unwrap().clone();
        if direction == &'L' {
            key = node.left;
        } else {
            key = node.right;
        }
        step_count += 1;
        if key.ends_with("Z") {
            break;
        }
    }
    step_count
}

fn part_two(lines: &[String]) -> i64 {
    let mut step_count: i64 = 0;
    let directions: Vec<char> = get_directions_from_lines(&lines);
    let hashmap: HashMap<String, Node> = get_hashmap_from_lines(&lines);
    let mut keys = get_starting_keys_from_hashmap(&hashmap);
    for direction in directions.iter().cycle() {
        step_count += 1;
        keys = get_new_keys_from_hashmap(&hashmap, &keys, &direction);
        if check_keys_end_with_z(&keys) {
            break;
        }
    }
    println!("step_count: {}", step_count);
    step_count
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn get_least_common_multiple(array: &[i64]) -> i64 {
    let mut lcm: i64 = array[0];
    for i in 1..array.len() {
        lcm = lcm * array[i] / gcd(lcm, array[i]);
    }
    lcm
}

fn part_two_optimized(lines: &[String]) -> i64 {
    let mut step_array: Vec<i64> = Vec::new();
    let directions: Vec<char> = get_directions_from_lines(&lines);
    let hashmap: HashMap<String, Node> = get_hashmap_from_lines(&lines);
    let keys = get_starting_keys_from_hashmap(&hashmap);
    println!("# keys: {}", keys.len());
    for key in keys.iter() {
        step_array.push(count_steps_for_key(key.to_string(), &hashmap, &directions));
    }
    let lcm: i64 = get_least_common_multiple(&step_array);
    println!("lcm: {}", lcm);
    lcm
}

fn main() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_eight_input.txt");
    part_one(&lines);
    //part_two(&lines);
    part_two_optimized(&lines);
}

#[test]
fn test_get_least_common_multiple() {
    let array: [i64; 3] = [2, 3, 4];
    assert_eq!(get_least_common_multiple(&array), 12);
    let array: [i64; 3] = [2, 7, 3];
    assert_eq!(get_least_common_multiple(&array), 42);
}

#[test]
fn test_part_one() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_eight_test_input.txt");
    assert_eq!(part_one(&lines), 6);
}

#[test]
fn test_part_two() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_eight_part_two_test.txt");
    assert_eq!(part_two(&lines), 6);
}

#[test]
fn test_part_two_optimized() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_eight_part_two_test.txt");
    assert_eq!(part_two_optimized(&lines), 6);
}
