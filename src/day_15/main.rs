use std::collections::HashMap;

use advent2023::advent_utils::get_lines_from_filepath;

#[derive(Debug, Clone)]
pub struct SequenceStep {
    id: u128,
    label: String,
    modifier: char,
    full: String,
}

impl SequenceStep {
    fn get_focal_length(&self) -> u128 {
        self.full
            .chars()
            .rev()
            .next()
            .unwrap()
            .to_digit(10)
            .unwrap() as u128
    }
}

impl SequenceStep {
    fn from_string(s: &str) -> SequenceStep {
        let id = hash_string_ignoring_operator(s);

        let mut label_chars: Vec<char> = Vec::new();
        for c in s.chars() {
            if c == '=' || c == '-' {
                break;
            }
            label_chars.push(c);
        }
        let value = label_chars.into_iter().collect();
        let modifier = s.chars().filter(|c| c == &'=' || c == &'-').next().unwrap();
        SequenceStep {
            id,
            label: value,
            modifier,
            full: s.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Box {
    labels: Vec<SequenceStep>,
}

impl Box {
    fn from_label(label: SequenceStep) -> Box {
        Box {
            labels: vec![label],
        }
    }
}

impl Box {
    fn update_labels(&mut self, label: &SequenceStep) {
        if label.modifier == '=' {
            // check if there is a label that has the same id
            let mut found = false;
            let mut new_labels: Vec<SequenceStep> = vec![];
            for l in self.labels.iter_mut() {
                if l.label == label.label {
                    new_labels.push(label.clone());
                    found = true;
                } else {
                    new_labels.push(l.clone());
                }
            }
            if !found {
                new_labels.push(label.clone());
            }
            self.labels = new_labels;
        } else {
            // check if there is a label that has the same id and remove it
            let mut new_labels: Vec<SequenceStep> = vec![];
            for l in self.labels.iter() {
                if l.label != label.label {
                    new_labels.push(l.clone());
                }
            }
            self.labels = new_labels;
        }
    }
}

fn part_one(lines: &[String]) -> u128 {
    let mut hash_sum = 0;
    for line in lines.iter() {
        for s in line.split(",") {
            hash_sum += hash_string(s);
        }
    }

    hash_sum
}

fn hash_string(s: &str) -> u128 {
    let mut hash = 0;
    for c in s.chars() {
        hash += c as u128;
        hash *= 17;
        hash = hash % 256;
    }
    hash
}

fn hash_string_ignoring_operator(s: &str) -> u128 {
    let mut hash = 0;
    for c in s.chars() {
        if c == '=' || c == '-' {
            break;
        }
        hash += c as u128;
        hash *= 17;
        hash = hash % 256;
    }
    hash
}

fn part_two(lines: &[String]) -> u128 {
    let mut box_hashmap: HashMap<u128, Box> = HashMap::new();
    for line in lines.iter() {
        for s in line.split(",") {
            let step = SequenceStep::from_string(s);
            if box_hashmap.contains_key(&step.id) {
                if let Some(x) = box_hashmap.get_mut(&step.id) {
                    x.update_labels(&step)
                }
            } else if step.modifier == '=' {
                box_hashmap.insert(step.id, Box::from_label(step));
            }
        }
    }
    let mut sum = 0;
    for (key, box_value) in box_hashmap.iter() {
        for (i, label) in box_value.labels.iter().enumerate() {
            sum += (key + 1) * (i as u128 + 1) * label.get_focal_length();
        }
    }
    sum
}

fn main() {
    let lines = get_lines_from_filepath("data/day_15_input.txt");
    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}

#[test]
fn test_hash_string() {
    assert_eq!(hash_string("rn=1"), 30);
    assert_eq!(hash_string_ignoring_operator("rn=1"), 0)
}

#[test]
fn test_part_one() {
    let lines = get_lines_from_filepath("data/day_15_test.txt");
    assert_eq!(part_one(&lines), 1320);
}

#[test]
fn test_get_focal_length() {
    let s = SequenceStep::from_string("rn=1");
    assert_eq!(s.get_focal_length(), 1);
}

#[test]
fn test_update_box() {
    let s = SequenceStep::from_string("rn=1");
    let mut b = Box::from_label(s);
    let z = SequenceStep::from_string("rn=0");
    b.update_labels(&z);
    for l in b.labels.iter() {
        assert_eq!(l.get_focal_length(), 0);
    }
}

#[test]
fn test_update_box_removing() {
    let s = SequenceStep::from_string("rn=1");
    let mut b = Box::from_label(s);
    let z = SequenceStep::from_string("rn-");
    b.update_labels(&z);
    assert_eq!(b.labels.len(), 0);
}

#[test]
fn test_update_box_no_value() {
    let s = SequenceStep::from_string("rn=1");
    let mut b = Box::from_label(s);
    let z = SequenceStep::from_string("cm-");
    b.update_labels(&z);
    assert_eq!(b.labels.len(), 1);
}

#[test]
fn test_update_box_adding() {
    let s = SequenceStep::from_string("rn=1");
    let mut b = Box::from_label(s);
    let z = SequenceStep::from_string("by=4");
    b.update_labels(&z);
    assert_eq!(b.labels.len(), 2);
}

#[test]
fn test_part_two() {
    let lines = get_lines_from_filepath("data/day_15_test.txt");
    assert_eq!(part_two(&lines), 145);
}
