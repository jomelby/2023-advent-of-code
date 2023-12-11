use advent2023::advent_utils::get_lines_from_filepath;

fn find_s_coordinates(lines: &[String]) -> Vec<(usize, usize)> {
    let mut s_coordinates: Vec<(usize, usize)> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                s_coordinates.push((x, y));
            }
        }
    }
    s_coordinates
}

fn get_surrounding_coordinates(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> Vec<(usize, usize)> {
    let mut surrounding_coordinates: Vec<(usize, usize)> = Vec::new();
    if x > 0 {
        surrounding_coordinates.push((x - 1, y));
    }
    if x < width - 1 {
        surrounding_coordinates.push((x + 1, y));
    }
    if y > 0 {
        surrounding_coordinates.push((x, y - 1));
    }
    if y < height - 1 {
        surrounding_coordinates.push((x, y + 1));
    }
    surrounding_coordinates
}

fn get_path(mut path: Vec<(usize, usize)>, lines: &[String]) -> Vec<(usize, usize)> {
    let (x, y) = path[path.len() - 1];
    let mut next_coordinates: (usize, usize) = (x, y);
    let direction = lines[y].chars().nth(x).unwrap();
    let x_diff: i128 = x as i128 - path[path.len() - 2].0 as i128;
    let y_diff: i128 = y as i128 - path[path.len() - 2].1 as i128;
    match direction {
        'S' => {
            return path;
        }
        '|' => {
            if y_diff > 0 {
                next_coordinates.1 += 1;
            } else if y_diff < 0 {
                next_coordinates.1 -= 1;
            } else {
                return path;
            }
        }
        '-' => {
            if x_diff > 0 {
                next_coordinates.0 += 1;
            } else if x_diff < 0 {
                next_coordinates.0 -= 1;
            } else {
                return path;
            }
        }
        'L' => {
            if y_diff > 0 {
                next_coordinates.0 += 1;
            } else if x_diff < 0 {
                next_coordinates.1 -= 1;
            } else {
                return path;
            }
        }
        '7' => {
            if y_diff < 0 {
                next_coordinates.0 -= 1;
            } else if x_diff > 0 {
                next_coordinates.1 += 1;
            } else {
                return path;
            }
        }
        'F' => {
            if y_diff < 0 {
                next_coordinates.0 += 1;
            } else if x_diff < 0 {
                next_coordinates.1 += 1;
            } else {
                return path;
            }
        }
        'J' => {
            if y_diff > 0 {
                next_coordinates.0 -= 1;
            } else if x_diff > 0 {
                next_coordinates.1 -= 1;
            } else {
                return path;
            }
        }
        _ => {
            return path;
        }
    }
    path.push(next_coordinates);
    return get_path(path, lines);
}

fn part_one(lines: &[String]) -> i128 {
    let s_coordinates = find_s_coordinates(lines);
    let surrounding_coordinates = get_surrounding_coordinates(
        s_coordinates[0].0,
        s_coordinates[0].1,
        lines[0].len(),
        lines.len(),
    );
    let mut path_lengths: Vec<i128> = Vec::new();
    for (x, y) in surrounding_coordinates {
        let mut path: Vec<(usize, usize)> = Vec::new();
        path.push(s_coordinates[0]);
        path.push((x, y));
        let surrounding_char = lines[y].chars().nth(x).unwrap();
        if lines[y].chars().nth(x).unwrap() != '.' {
            path = get_path(path, lines);
        }
        if path[0] == path[path.len() - 1] {
            path_lengths.push(path.len() as i128);
        }
    }

    return path_lengths.iter().max().unwrap().clone() / 2;
}

fn main() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_ten_input.txt");
    let part_one_answer = part_one(&lines);
    println!("Part One Answer: {}", part_one_answer);
}

#[test]
fn test_part_one() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_ten_test.txt");
    assert_eq!(part_one(&lines), 8);
}

#[test]
fn test_part_one_easy() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_ten_test_easy.txt");
    assert_eq!(part_one(&lines), 4);
}

#[test]
fn test_s_coordinate() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_ten_test.txt");
    let s_coordinates = find_s_coordinates(&lines);
    assert_eq!(s_coordinates.len(), 1);
    assert_eq!(s_coordinates[0], (0, 2));
}
