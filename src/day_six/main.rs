use advent2023::advent_utils::get_lines_from_filepath;

pub struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn number_winning_strategies(&self) -> i64 {
        let mut winning_strategies: i64 = 0;
        for button_push_time in 1..self.time + 1 {
            let distance = (self.time - button_push_time) * button_push_time;
            if distance > self.distance {
                winning_strategies += 1;
            }
        }
        winning_strategies
    }
}

fn get_races(lines: &[String]) -> Vec<Race> {
    let times: Vec<String> = lines[0]
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()[1..]
        .to_vec();
    let distances: Vec<String> = lines[1]
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()[1..]
        .to_vec();
    let races: Vec<Race> = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Race {
            time: time.parse::<i64>().unwrap(),
            distance: distance.parse::<i64>().unwrap(),
        })
        .collect();
    races
}

fn get_races_conat(lines: &[String]) -> Vec<Race> {
    let time: String = lines[0]
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()[1..]
        .to_vec()
        .concat();
    let distance: String = lines[1]
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()[1..]
        .to_vec()
        .concat();
    let races: Vec<Race> = vec![Race {
        time: time.parse::<i64>().unwrap(),
        distance: distance.parse::<i64>().unwrap(),
    }];
    races
}

fn part_one(lines: &[String]) {
    let mut product: i64 = 1;
    let races: Vec<Race> = get_races(lines);
    for race in races {
        product *= race.number_winning_strategies();
    }
    println!("Part one: {}", product);
}

fn part_two(lines: &[String]) {
    let mut product: i64 = 1;
    let races: Vec<Race> = get_races_conat(lines);
    for race in races {
        product *= race.number_winning_strategies();
    }
    println!("Part two: {}", product);
}
fn main() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_six_input.txt");
    part_one(&lines);
    part_two(&lines);
}
