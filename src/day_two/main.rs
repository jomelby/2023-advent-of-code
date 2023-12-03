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

impl ColorSet {
    fn power(&self) -> i32 {
        self.green * self.red * self.blue
    }
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

fn return_max_color_set(color_set: &ColorSet, other: &ColorSet) -> ColorSet {
    ColorSet {
        green: std::cmp::max(color_set.green, other.green),
        red: std::cmp::max(color_set.red, other.red),
        blue: std::cmp::max(color_set.blue, other.blue),
    }
}

fn get_sets_from_line(line: &str) -> Vec<&str> {
    let game_sets: Vec<&str> = line.split(":").collect();
    let sets: Vec<&str> = game_sets[1].split(";").collect();
    sets
}

fn main() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_two_input.txt");
    let mut power_sum = 0;
    for line in lines.iter() {
        let game_number = get_game_number(line);
        println!("{} ", game_number);
        let mut max_set = ColorSet {
            green: 0,
            red: 0,
            blue: 0,
        };
        for set in get_sets_from_line(line).iter() {
            let color_set: ColorSet = get_color_set_from_set(set);
            max_set = return_max_color_set(&max_set, &color_set);
        }
        power_sum += max_set.power();
    }
    println!("{}", power_sum)
}
