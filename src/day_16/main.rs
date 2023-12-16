use std::{
    char,
    collections::{HashMap, HashSet},
};

use advent2023::advent_utils::get_lines_from_filepath;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

fn get_contraption_from_lines(lines: &[String]) -> HashMap<(i128, i128), char> {
    let mut contraption_map: HashMap<(i128, i128), char> = HashMap::new();
    for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            contraption_map.insert((col as i128, row as i128), char);
        }
    }
    contraption_map
}

fn move_beam(
    contraption_map: &HashMap<(i128, i128), char>,
    mut current_position: (i128, i128),
    mut previous_position: (i128, i128),
    sampled_positions: &mut HashSet<((i128, i128), (i128, i128))>,
) -> () {
    if current_position == previous_position {
        return;
    }
    let max_x = contraption_map.keys().map(|(x, _)| x).max().unwrap();
    let max_y = contraption_map.keys().map(|(_, y)| y).max().unwrap();
    if current_position.0 > *max_x
        || current_position.1 > *max_y
        || current_position.0 < 0
        || current_position.1 < 0
    {
        return;
    }
    if sampled_positions.contains(&(previous_position, current_position)) {
        return;
    }
    let x_direction: i128 = current_position.0 - previous_position.0;
    let y_direction: i128 = current_position.1 - previous_position.1;
    let mut new_position: (i128, i128) = (-1, -1);
    sampled_positions.insert((previous_position, current_position));
    let current_position_char: &char = contraption_map
        .get(&(current_position.0, current_position.1))
        .unwrap();
    if x_direction != 0 {
        // beam is going left or right
        if current_position_char == &'.' || current_position_char == &'-' {
            // beam is not at the bottom of the contraption
            new_position = (current_position.0 + 1 * x_direction, current_position.1);
            previous_position = current_position;
            current_position = new_position;
            move_beam(
                contraption_map,
                current_position,
                previous_position,
                sampled_positions,
            );
        } else if current_position_char == &'/' {
            new_position = (current_position.0, current_position.1 - 1 * x_direction);
            previous_position = current_position;
            current_position = new_position;
            move_beam(
                contraption_map,
                current_position,
                previous_position,
                sampled_positions,
            );
        } else if current_position_char == &'\\' {
            new_position = (current_position.0, current_position.1 + 1 * x_direction);
            previous_position = current_position;
            current_position = new_position;
            move_beam(
                contraption_map,
                current_position,
                previous_position,
                sampled_positions,
            );
        } else if current_position_char == &'|' {
            // beam is being split up and down
            new_position = (current_position.0, current_position.1 + 1);
            previous_position = current_position;
            current_position = new_position;
            move_beam(
                contraption_map,
                current_position,
                previous_position,
                sampled_positions,
            );
            new_position = (current_position.0, current_position.1 - 2);
            current_position = new_position;
            move_beam(
                contraption_map,
                current_position,
                previous_position,
                sampled_positions,
            );
        } else {
            // beam is being split up and down
            println!("unexpected character: {}", current_position_char);
        }
    } else {
        // beam is going up or down
        if current_position_char == &'.' || current_position_char == &'|' {
            new_position = (current_position.0, current_position.1 + 1 * y_direction);
            previous_position = current_position;
            current_position = new_position;
            move_beam(
                contraption_map,
                current_position,
                previous_position,
                sampled_positions,
            );
        } else if current_position_char == &'/' {
            new_position = (current_position.0 - 1 * y_direction, current_position.1);
            previous_position = current_position;
            current_position = new_position;
            move_beam(
                contraption_map,
                current_position,
                previous_position,
                sampled_positions,
            );
        } else if current_position_char == &'\\' {
            new_position = (current_position.0 + 1 * y_direction, current_position.1);
            previous_position = current_position;
            current_position = new_position;
            move_beam(
                contraption_map,
                current_position,
                previous_position,
                sampled_positions,
            );
        } else if current_position_char == &'-' {
            // beam is being split up and down
            new_position = (current_position.0 + 1, current_position.1);
            previous_position = current_position;
            current_position = new_position;
            move_beam(
                contraption_map,
                current_position,
                previous_position,
                sampled_positions,
            );
            new_position = (current_position.0 - 2, current_position.1);
            current_position = new_position;
            move_beam(
                contraption_map,
                current_position,
                previous_position,
                sampled_positions,
            );
        }
    }
}

fn part_one(lines: &Vec<String>) -> i128 {
    let contraption_map = get_contraption_from_lines(&lines);
    let previous_position: (i128, i128) = (0, 0);
    let current_position: (i128, i128) = (1, 0);
    let mut sampled_positions: HashSet<((i128, i128), (i128, i128))> = HashSet::new();
    move_beam(
        &contraption_map,
        current_position,
        previous_position,
        &mut sampled_positions,
    );
    //need to build a hashset of all the positions that the beam has been in
    let mut unique_positions: HashSet<(i128, i128)> = HashSet::new();
    for (pos1, pos2) in sampled_positions {
        unique_positions.insert(pos1);
        unique_positions.insert(pos2);
    }
    unique_positions.len() as i128
}

fn get_starting_positions(
    contraption_map: &HashMap<(i128, i128), char>,
) -> Vec<((i128, i128), (i128, i128))> {
    let mut starting_positions: Vec<((i128, i128), (i128, i128))> = Vec::new();
    let max_x = contraption_map.keys().map(|(x, _)| x).max().unwrap();
    let max_y = contraption_map.keys().map(|(_, y)| y).max().unwrap();
    for x in 0..max_x + 1 {
        starting_positions.push(((x, 0), (x, 1)));
        starting_positions.push(((x, *max_y), (x, max_y - 1)));
    }
    for y in 0..max_y + 1 {
        starting_positions.push(((0, y), (1, y)));
        starting_positions.push(((*max_x, y), (max_x - 1, y)));
    }
    starting_positions
}

fn part_two(lines: &Vec<String>) -> i128 {
    let contraption_map = get_contraption_from_lines(&lines);
    let starting_positions: Vec<((i128, i128), (i128, i128))> =
        get_starting_positions(&contraption_map);
    let mut max_unique_positions: i128 = 0;
    // let unique_positions_list: Vec<i128> = starting_positions
    //     .par_iter()
    //     .map(|x| {
    //         let mut sampled_positions: HashSet<((i128, i128), (i128, i128))> = HashSet::new();
    //         let previous_position = x.0;
    //         let current_position = x.1;
    //         move_beam(
    //             &contraption_map,
    //             current_position,
    //             previous_position,
    //             &mut sampled_positions,
    //         );
    //         //need to build a hashset of all the positions that the beam has been in
    //         let mut unique_positions: HashSet<(i128, i128)> = HashSet::new();
    //         for (pos1, pos2) in sampled_positions {
    //             unique_positions.insert(pos1);
    //             unique_positions.insert(pos2);
    //         }
    //         unique_positions.len() as i128
    //     })
    //     .collect();
    for (x, (previous_position, current_position)) in starting_positions.iter().enumerate() {
        let mut sampled_positions: HashSet<((i128, i128), (i128, i128))> = HashSet::new();
        move_beam(
            &contraption_map,
            *current_position,
            *previous_position,
            &mut sampled_positions,
        );
        //need to build a hashset of all the positions that the beam has been in
        let mut unique_positions: HashSet<(i128, i128)> = HashSet::new();
        for (pos1, pos2) in sampled_positions {
            unique_positions.insert(pos1);
            unique_positions.insert(pos2);
        }
        println!("{}: {}", x, unique_positions.len());
        if unique_positions.len() as i128 > max_unique_positions {
            max_unique_positions = unique_positions.len() as i128;
        }
    }
    max_unique_positions
}

fn main() {
    let lines = get_lines_from_filepath("data/day_16_input.txt");
    // println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}

#[test]
fn test_part_one() {
    let lines = get_lines_from_filepath("data/day_16_test_input.txt");
    println!("{:?}", lines);
    assert_eq!(part_one(&lines), 46);
}

#[test]
fn test_part_two() {
    let lines = get_lines_from_filepath("data/day_16_test_input.txt");
    println!("{:?}", lines);
    assert_eq!(part_two(&lines), 51);
}

#[test]
fn test_part_one_shortened() {
    let lines = get_lines_from_filepath("data/day_16_test_input_shortened.txt");
    println!("{:?}", lines);
    assert_eq!(part_one(&lines), 8);
}
