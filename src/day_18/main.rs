use std::collections::{HashMap, HashSet};

use advent2023::advent_utils::get_lines_from_filepath;

pub struct DigPlanStep {
    direction: char,
    distance: i128,
    color_code: String,
}

impl DigPlanStep {
    fn from_line(line: String) -> DigPlanStep {
        let line_split: Vec<&str> = line.split(" ").collect();
        let direction: char = line_split[0].chars().next().unwrap();
        let distance: i128 = line_split[1].to_string().parse::<i128>().unwrap();
        let color_code: String = line_split[2].to_string();
        DigPlanStep {
            direction,
            distance,
            color_code,
        }
    }
}

fn row_cube_fill(cleared_cubes_row: Vec<&(i128, i128)>) -> i128 {
    let mut filled_cubes: i128 = 0;
    println!("{:?}", cleared_cubes_row);
    let mut skip_next: bool = false;
    let mut x_0: i128 = cleared_cubes_row[0].0;
    let mut previous_x: i128 = cleared_cubes_row[0].0;
    for i in 1..cleared_cubes_row.len() {
        if i == cleared_cubes_row.len() - 1 {
            filled_cubes += cleared_cubes_row[i].0 - x_0 + 1;
        } else if cleared_cubes_row[i].0 - previous_x == 1 {
            previous_x = cleared_cubes_row[i].0;
            continue;
        } else if skip_next {
            skip_next = false;
            x_0 = cleared_cubes_row[i].0;
            previous_x = cleared_cubes_row[i].0;
            continue;
        } else {
            filled_cubes += cleared_cubes_row[i].0 - x_0 + 1;
            previous_x = cleared_cubes_row[i + 1].0;
            skip_next = true;
        }
    }
    //     if i + 1 == cleared_cubes_row.len() - 1 {
    //         filled_cubes += cleared_cubes_row[i + 1].0 - cleared_cubes_row[i].0 + 1;
    //     } else if !skip_next {
    //         let cube_distance = cleared_cubes_row[i + 1].0 - cleared_cubes_row[i].0;
    //         if cube_distance == 1 {
    //             filled_cubes += 1;
    //             if i + 2 < cleared_cubes_row.len()
    //                 && cleared_cubes_row[i + 2].0 - cleared_cubes_row[i + 1].0 == 1
    //             {
    //                 continue;
    //             } else {
    //                 skip_next = true;
    //             }
    //         } else if cube_distance > 1 {
    //             filled_cubes += cube_distance;
    //         }
    //     } else {
    //         skip_next = false;
    //     }
    // }
    println!("{}", filled_cubes);
    filled_cubes
}

fn get_area_cleared_cubes_area(cleared_cubes: &Vec<(i128, i128)>) -> i128 {
    let mut area: i128 = 0;
    let min_y: i128 = cleared_cubes.iter().map(|c| c.1).min().unwrap();
    let max_y: i128 = cleared_cubes.iter().map(|c| c.1).max().unwrap();
    for i in min_y..=max_y {
        // rows that match the y coordinate
        let mut matching_rows: Vec<&(i128, i128)> = cleared_cubes
            .iter()
            .filter(|c| c.1 == i)
            .collect::<Vec<&(i128, i128)>>();
        matching_rows.sort_by(|a, b| a.0.cmp(&b.0));
        area += row_cube_fill(matching_rows);
    }
    area
}

fn shoelace_area(cleared_cubes: &Vec<(i128, i128)>) -> i128 {
    let mut area: i128 = 0;
    let x_coordinates: Vec<i128> = cleared_cubes.iter().map(|c| c.0).collect();
    let y_coordinates: Vec<i128> = cleared_cubes.iter().map(|c| c.1).collect();
    for i in 0..x_coordinates.len() - 1 {
        area += x_coordinates[i] * y_coordinates[i + 1] * 4
            - x_coordinates[i + 1] * y_coordinates[i] * 4;
    }
    area.abs() / 2
}

fn part_one(lines: &[String]) -> i128 {
    let mut dig_coordinates: (i128, i128) = (0, 0);
    let mut cleared_cubes: Vec<(i128, i128)> = Vec::new();
    // cleared_cubes.push(dig_coordinates);
    for line in lines.iter() {
        let dig_plan_step = DigPlanStep::from_line(line.to_string());
        let mut x_change = 0;
        let mut y_change = 0;
        match dig_plan_step.direction {
            'R' => {
                x_change = 1;
            }
            'L' => {
                x_change = -1;
            }
            'D' => {
                y_change = 1;
            }
            'U' => {
                y_change = -1;
            }
            _ => panic!("Invalid direction"),
        }
        // for shoelace algorithm just store the vertices
        // dig_coordinates.0 += x_change * dig_plan_step.distance;
        // dig_coordinates.1 += y_change * dig_plan_step.distance;
        // cleared_cubes.push(dig_coordinates);
        for _ in 0..dig_plan_step.distance {
            dig_coordinates.0 += x_change;
            dig_coordinates.1 += y_change;
            cleared_cubes.push(dig_coordinates);
        }
    }
    // println!("{:?}", cleared_cubes);
    // get_area_cleared_cubes_area(&cleared_cubes)
    shoelace_area(&cleared_cubes)
}

fn main() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_18_input.txt");
    let result = part_one(&lines);
    println!("{}", result);
    // let result_2 = part_two(&lines);
    // println!("{}", result_2);
}

#[test]
fn test_part_one() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_18_test.txt");
    //36353 is too high
    let result = part_one(&lines);
    assert_eq!(result, 62);
}

#[test]
fn test_shoelace_area() {
    let mut cleared_cubes: Vec<(i128, i128)> = Vec::new();
    cleared_cubes.push((0, 0));
    cleared_cubes.push((1, 0));
    cleared_cubes.push((1, 1));
    cleared_cubes.push((0, 1));
    let result = shoelace_area(&cleared_cubes);
    assert_eq!(result, 4);
    let mut cleared_cubes: Vec<(i128, i128)> = Vec::new();
    cleared_cubes.push((0, 0));
    cleared_cubes.push((2, 0));
    cleared_cubes.push((2, 1));
    cleared_cubes.push((0, 1));
    let result = shoelace_area(&cleared_cubes);
    assert_eq!(result, 6);
}
