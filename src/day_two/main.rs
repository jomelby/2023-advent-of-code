use advent2023::advent_utils::get_lines_from_filepath;

fn get_game_number(line: &str) -> i32 {
    let words: Vec<&str> = line.split_whitespace().collect();
    let game_number = words[1].replace(":", "").parse::<i32>().unwrap();
    game_number
}

#[derive(Debug)]
pub struct ColorSet {
    green: i32,
    red: i32,
    blue: i32,
}

impl PartialOrd for ColorSet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.green == other.green && self.red == other.red && self.blue == other.blue {
            Some(std::cmp::Ordering::Equal)
        } else if self.green >= other.green && self.red >= other.red && self.blue >= other.blue {
            Some(std::cmp::Ordering::Greater)
        } else if self.green < other.green && self.red < other.red && self.blue < other.blue {
            Some(std::cmp::Ordering::Less)
        } else {
            None
        }
    }
}
impl PartialEq for ColorSet {
    fn eq(&self, other: &Self) -> bool {
        self.green == other.green && self.red == other.red && self.blue == other.blue
    }
}
// " 1 green, 2 red, 3 blue"
fn get_color_set_from_set(set_str: &str) -> ColorSet {
    let mut green: i32 = 0;
    let mut red: i32 = 0;
    let mut blue: i32 = 0;
    for number_color in set_str.split(",") {
        let number_color: Vec<&str> = number_color.split_whitespace().collect();
        let number = number_color[0].parse::<i32>().unwrap();
        let color = number_color[1];
        match color {
            "green" => green = number,
            "red" => red = number,
            "blue" => blue = number,
            _ => println!("Unknown color: {}", color),
        }
    }
    ColorSet { green, red, blue }
}

fn get_sets_from_line(line: &str) -> Vec<&str> {
    let game_sets: Vec<&str> = line.split(":").collect();
    let sets: Vec<&str> = game_sets[1].split(";").collect();
    sets
}

fn main() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_two_input.txt");
    let ref_set: ColorSet = ColorSet {
        green: 13,
        red: 12,
        blue: 14,
    };
    let example_set = ColorSet {
        green: 6,
        red: 12,
        blue: 8,
    };
    println!("{}", (ref_set >= example_set));
    let mut game_number_sum = 0;
    for line in lines.iter() {
        let game_number = get_game_number(line);
        println!("{} ", game_number);
        let mut possible: bool = true;
        for set in get_sets_from_line(line).iter() {
            if !possible {
                continue;
            }
            let color_set: ColorSet = get_color_set_from_set(set);
            println!("{:?} ", color_set);
            println!("{} ", (ref_set >= color_set));
            possible = ref_set >= color_set;
        }
        if possible {
            println!("Possible:{}", possible);
            game_number_sum += game_number;
        }
    }
    println!("{}", game_number_sum)
}
