use advent2023::advent_utils::get_lines_from_filepath;

#[derive(Debug, Clone)]
pub struct Galaxy {
    x: i128,
    y: i128,
}

fn calculate_galaxy_distance(galaxy: &Galaxy, other_galaxy: &Galaxy) -> i128 {
    (galaxy.x - other_galaxy.x).abs() + (galaxy.y - other_galaxy.y).abs()
}

fn expand_universe(lines: &[String]) -> Vec<String> {
    let mut new_lines: Vec<String> = Vec::new();
    let mut columns_to_insert: Vec<usize> = Vec::new();
    for i in 0..lines[0].len() {
        let mut insert_column = true;
        for line in lines.iter() {
            if line.chars().nth(i).unwrap() == '#' {
                insert_column = false;
                break;
            }
        }
        if insert_column {
            columns_to_insert.push(i);
        }
    }
    for line in lines.iter() {
        let mut new_line = line.clone();
        for (i, columns_to_insert) in columns_to_insert.iter().enumerate() {
            let mut chars: Vec<char> = new_line.chars().collect();
            chars.insert(*columns_to_insert + i + 1, '.');
            new_line = chars.into_iter().collect();
        }
        if line.contains("#") {
            new_lines.push(new_line);
        } else {
            new_lines.push(new_line.clone());
            new_lines.push(new_line.clone());
        }
    }
    new_lines
}

fn get_galaxies(lines: &[String]) -> Vec<Galaxy> {
    let mut galaxies: Vec<Galaxy> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Galaxy {
                    x: x as i128,
                    y: y as i128,
                });
            }
        }
    }
    galaxies
}

fn part_one(lines: &[String]) -> i128 {
    let mut sum_distance: i128 = 0;
    let expanded_universe = expand_universe(lines);
    let galaxies = get_galaxies(&expanded_universe);
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            sum_distance += calculate_galaxy_distance(&galaxies[i], &galaxies[j]);
        }
    }
    sum_distance
}

fn expand_universe_part_two(lines: &[String], multiplier: i128) -> Vec<String> {
    let mut new_lines: Vec<String> = Vec::new();
    let mut columns_to_insert: Vec<usize> = Vec::new();
    for i in 0..lines[0].len() {
        let mut insert_column = true;
        for line in lines.iter() {
            if line.chars().nth(i).unwrap() == '#' {
                insert_column = false;
                break;
            }
        }
        if insert_column {
            columns_to_insert.push(i);
        }
    }
    for line in lines.iter() {
        let mut new_line = line.clone();
        for (i, columns_to_insert) in columns_to_insert.iter().enumerate() {
            let range_end = columns_to_insert + i * multiplier as usize + 1;
            let shortened_line = new_line[0..range_end].to_string();
            let line_end = new_line[range_end..].to_string();
            new_line = format!(
                "{}{}{}",
                shortened_line,
                ".".repeat(multiplier as usize),
                line_end
            );
        }
        if line.contains("#") {
            new_lines.push(new_line.clone());
        } else {
            new_lines.push(new_line.clone());
            for _ in 0..multiplier {
                new_lines.push(new_line.clone());
            }
        }
    }
    new_lines
}

fn row_to_expand(lines: &[String]) -> Vec<usize> {
    let mut rows_to_expand: Vec<usize> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if line.contains("#") {
            continue;
        }
        rows_to_expand.push(i);
    }
    rows_to_expand
}

fn col_to_expand(lines: &[String]) -> Vec<usize> {
    let mut columns_to_insert: Vec<usize> = Vec::new();
    for i in 0..lines[0].len() {
        let mut insert_column = true;
        for line in lines.iter() {
            if line.chars().nth(i).unwrap() == '#' {
                insert_column = false;
                break;
            }
        }
        if insert_column {
            columns_to_insert.push(i);
        }
    }
    columns_to_insert
}

fn update_galaxy(
    galaxy: &mut Galaxy,
    rows_to_expand: &[usize],
    columns_to_expand: &[usize],
    multiplier: i128,
) {
    for (i, row) in rows_to_expand.iter().enumerate() {
        if galaxy.y < *row as i128 {
            galaxy.y += (i as i128) * multiplier;
            break;
        } else if i == rows_to_expand.len() - 1 {
            galaxy.y += (i as i128 + 1) * multiplier;
        }
    }
    for (i, column) in columns_to_expand.iter().enumerate() {
        if galaxy.x < *column as i128 {
            galaxy.x += (i as i128) * multiplier;
            break;
        } else if i == columns_to_expand.len() - 1 {
            galaxy.x += (i as i128 + 1) * multiplier;
        }
    }
}

fn part_two(lines: &[String], expansion_n: i128) -> i128 {
    let mut sum_distance: i128 = 0;
    let galaxies = get_galaxies(lines);
    let rows_to_expand = row_to_expand(lines);
    let columns_to_expand = col_to_expand(lines);
    let mut expanded_galaxies: Vec<Galaxy> = Vec::new();
    for galaxy in galaxies.iter() {
        let mut new_galaxy = galaxy.clone();
        update_galaxy(
            &mut new_galaxy,
            &rows_to_expand,
            &columns_to_expand,
            expansion_n,
        );
        expanded_galaxies.push(new_galaxy);
    }

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            sum_distance += calculate_galaxy_distance(&expanded_galaxies[i], &expanded_galaxies[j]);
        }
    }
    sum_distance
}

fn main() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_eleven.txt");
    println!("Part one answer: {}", part_one(&lines));
    println!("Part two answer: {}", part_two(&lines, 999999));
}

#[test]
fn test_part_one() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_eleven_test.txt");
    assert_eq!(part_one(&lines), 374);
}

#[test]
fn test_galaxy_distance() {
    let galaxy = Galaxy { x: 1, y: 1 };
    let other_galaxy = Galaxy { x: 2, y: 2 };
    assert_eq!(calculate_galaxy_distance(&galaxy, &other_galaxy), 2);
}

#[test]
fn test_galaxy_expansion() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_eleven_test.txt");
    let answer: Vec<String> = get_lines_from_filepath("data/day_eleven_test expanded.txt");
    for (i, line) in expand_universe(&lines).iter().enumerate() {
        println!("{}", line);
        println!("{}", answer[i]);
        assert_eq!(line, &answer[i]);
    }
}

#[test]
fn test_galaxy_expansion_part_two() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_eleven_test.txt");
    let answer: Vec<String> = get_lines_from_filepath("data/day_eleven_test expanded_part_2.txt");
    for (i, line) in expand_universe_part_two(&lines, 2).iter().enumerate() {
        println!("{}", line);
        println!("{}", answer[i]);
        assert_eq!(line, &answer[i]);
    }
    let answer: Vec<String> = get_lines_from_filepath("data/day_eleven_test expanded.txt");
    for (i, line) in expand_universe_part_two(&lines, 1).iter().enumerate() {
        println!("{}", line);
        println!("{}", answer[i]);
        assert_eq!(line, &answer[i]);
    }
}

#[test]
fn test_part_two() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_eleven_test.txt");
    assert_eq!(part_two(&lines, 9), 8410);
}

#[test]
fn test_update_galaxy() {
    let mut galaxy = Galaxy { x: 1, y: 1 };
    let rows_to_expand = vec![0, 2, 3];
    let columns_to_expand = vec![0, 2, 3];
    update_galaxy(&mut galaxy, &rows_to_expand, &columns_to_expand, 2);
    assert_eq!(galaxy.x, 3);
    assert_eq!(galaxy.y, 3);
    let mut galaxy = Galaxy { x: 4, y: 4 };
    let rows_to_expand = vec![0, 2, 3];
    let columns_to_expand = vec![0, 2, 3];
    update_galaxy(&mut galaxy, &rows_to_expand, &columns_to_expand, 2);
    assert_eq!(galaxy.x, 10);
    assert_eq!(galaxy.y, 10);
    let mut galaxy = Galaxy { x: 0, y: 0 };
    let rows_to_expand = vec![1, 2, 3];
    let columns_to_expand = vec![1, 2, 3];
    update_galaxy(&mut galaxy, &rows_to_expand, &columns_to_expand, 2);
    assert_eq!(galaxy.x, 0);
    assert_eq!(galaxy.y, 0);
}
