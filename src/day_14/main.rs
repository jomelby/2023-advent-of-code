use advent2023::advent_utils::get_lines_from_filepath;

fn score_dish(dish: &[String]) -> i128 {
    let mut score: i128 = 0;
    for (i, line) in dish.iter().enumerate() {
        for char in line.chars() {
            if char == 'O' {
                score += dish.len() as i128 - i as i128;
            }
        }
    }
    score
}

fn transpose_dish(dish: &[String]) -> Vec<String> {
    let height = dish.len();
    let width = dish[0].len();

    (0..width)
        .map(|i| {
            (0..height)
                .map(|j| dish[j].chars().nth(i).unwrap())
                .collect()
        })
        .collect()
}

fn tilt_line(line: &str) -> String {
    let mut new_line = String::new();
    let rounded_rock_locations = line
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == 'O')
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    let cube_shaped_rock_locations = line
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    let mut new_rock_position = 0;
    let mut new_rock_positions: Vec<usize> = Vec::new();
    let mut last_rock_position = 1;
    if cube_shaped_rock_locations.len() > 0 {
        last_rock_position = cube_shaped_rock_locations[cube_shaped_rock_locations.len() - 1] + 1;
    }
    for rounded_rock in rounded_rock_locations.iter() {
        for (cube_i, cube_shaped_rock) in cube_shaped_rock_locations.clone().iter().enumerate() {
            if rounded_rock < &cube_shaped_rock {
                new_rock_positions.push(new_rock_position);
                new_rock_position += 1;
                break;
            } else if cube_i == cube_shaped_rock_locations.len() - 1 {
                new_rock_positions.push(last_rock_position);
                last_rock_position += 1;
            } else if new_rock_position <= cube_shaped_rock + 1 {
                new_rock_position = cube_shaped_rock + 1;
            }
        }
        if cube_shaped_rock_locations.len() == 0 {
            new_rock_positions.push(new_rock_position);
            new_rock_position += 1;
        }
    }
    for (i, c) in line.chars().enumerate() {
        if new_rock_positions.contains(&i) {
            new_line.push_str("O");
        } else if c == 'O' {
            new_line.push_str(".")
        } else {
            new_line.push_str(&c.to_string());
        }
    }
    new_line
}

fn tilt_line_simpler(line: &str) -> String {
    let mut new_line = line.to_string();
    for _ in 0..line.len() {
        for c in 0..line.len() {
            if new_line.chars().nth(c).unwrap() == 'O'
                && c > 0
                && new_line.chars().nth(c - 1).unwrap() == '.'
            {
                new_line.replace_range(c - 1..c + 1, "O.");
            }
        }
    }
    new_line
}

fn part_one(dish: &[String]) -> i128 {
    let mut initial_count = 0;
    for line in dish.iter() {
        for char in line.chars() {
            if char == 'O' {
                initial_count += 1;
            }
        }
    }
    let transposed_dish = transpose_dish(dish);
    let mut tilted_transposed_dish: Vec<String> = Vec::new();
    for line in transposed_dish.iter() {
        tilted_transposed_dish.push(tilt_line(line));
    }
    let untilted_dish = transpose_dish(&tilted_transposed_dish);
    let mut final_count = 0;
    for line in untilted_dish.iter() {
        for char in line.chars() {
            if char == 'O' {
                final_count += 1;
            }
        }
    }
    println!("{}", untilted_dish.join("\n"));
    println!("{} {}", initial_count, final_count);
    score_dish(&untilted_dish)
}

fn cycle_dish(dish: &[String]) -> Vec<String> {
    // north
    let mut north_tilted_dish: Vec<String> = Vec::new();
    let mut transposed_dish = transpose_dish(dish);
    for line in transposed_dish.iter() {
        north_tilted_dish.push(tilt_line_simpler(line));
    }
    north_tilted_dish = transpose_dish(&north_tilted_dish);
    //west
    let mut west_tilted_dish: Vec<String> = Vec::new();
    for line in north_tilted_dish.iter() {
        west_tilted_dish.push(tilt_line_simpler(line));
    }
    // south
    let mut south_tilted_dish: Vec<String> = Vec::new();
    transposed_dish = transpose_dish(&west_tilted_dish);
    for line in transposed_dish.iter() {
        south_tilted_dish.push(
            tilt_line_simpler(line.chars().rev().collect::<String>().as_str())
                .chars()
                .rev()
                .collect(),
        );
    }
    south_tilted_dish = transpose_dish(&south_tilted_dish);
    //east
    let mut east_tilted_dish: Vec<String> = Vec::new();
    for line in south_tilted_dish.iter() {
        east_tilted_dish.push(
            tilt_line_simpler(line.chars().rev().collect::<String>().as_str())
                .chars()
                .rev()
                .collect(),
        );
    }
    east_tilted_dish
}

fn part_two(dish: &[String]) -> i128 {
    let mut cycled_dish: Vec<String> = cycle_dish(&dish);
    let mut num_cycles = 1;
    for _ in 0..999999999 {
        num_cycles += 1;
        cycled_dish = cycle_dish(&cycled_dish);
    }
    println!("{}", num_cycles);
    score_dish(&cycled_dish)
}

fn main() {
    let lines = get_lines_from_filepath("data/day_14_input.txt");
    println!("Part One: {}", part_one(&lines));
    println!("Part Two: {}", part_two(&lines));
}

#[test]
fn test_part_one() {
    let dish = get_lines_from_filepath("data/day_14_test.txt");
    assert_eq!(part_one(&dish), 136)
}

#[test]
fn test_part_two() {
    let dish = get_lines_from_filepath("data/day_14_test.txt");
    assert_eq!(part_two(&dish), 64)
}

#[test]
fn test_dish_score() {
    let dish = get_lines_from_filepath("data/day_14_test_score.txt");
    assert_eq!(score_dish(&dish), 136)
}

#[test]
fn test_tilt_line() {
    let line = "...OOO".to_string();
    assert_eq!(tilt_line(&line), "OOO...");
    let line = "#..OOO".to_string();
    assert_eq!(tilt_line(&line), "#OOO..");
    let line = "#.O#..O".to_string();
    assert_eq!(tilt_line(&line), "#O.#O..");
    let line = "#.O#.OO".to_string();
    assert_eq!(tilt_line(&line), "#O.#OO.");
    let line = "#OO#.OO".to_string();
    assert_eq!(tilt_line(&line), "#OO#OO.");
}

#[test]
fn test_tilt_line_simpler() {
    let line = "...OOO".to_string();
    assert_eq!(tilt_line_simpler(&line), "OOO...");
    let line = "#..OOO".to_string();
    assert_eq!(tilt_line_simpler(&line), "#OOO..");
    let line = "#.O#..O".to_string();
    assert_eq!(tilt_line_simpler(&line), "#O.#O..");
    let line = "#.O#.OO".to_string();
    assert_eq!(tilt_line_simpler(&line), "#O.#OO.");
    let line = "#OO#.OO".to_string();
    assert_eq!(tilt_line_simpler(&line), "#OO#OO.");
}

#[test]
fn test_two_cycles() {
    let dish = get_lines_from_filepath("data/day_14_test.txt");
    let mut cycled_dish = cycle_dish(&dish);
    cycled_dish = cycle_dish(&cycled_dish);
    let answer = get_lines_from_filepath("data/day_14_test_two_cycles.txt");
    for (i, line) in cycled_dish.iter().enumerate() {
        assert_eq!(line, &answer[i]);
    }
}

#[test]
fn test_three_cycles() {
    let dish = get_lines_from_filepath("data/day_14_test.txt");
    let mut cycled_dish = cycle_dish(&dish);
    cycled_dish = cycle_dish(&cycled_dish);
    cycled_dish = cycle_dish(&cycled_dish);
    let answer = get_lines_from_filepath("data/day_14_test_three_cycles.txt");
    for (i, line) in cycled_dish.iter().enumerate() {
        assert_eq!(line, &answer[i]);
    }
}
