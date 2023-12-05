use advent2023::advent_utils::get_lines_from_filepath;

#[derive(Debug)]
pub struct Scratchcard {
    winning_numbers: Vec<i32>,
    play_numbers: Vec<i32>,
}

impl Scratchcard {
    fn from_line(line: &String) -> Self {
        // strip card number from the line
        let card_line: Vec<&str> = line.split(":").collect();
        // split the winning numbers from the play numbers
        let just_winning_numbers_and_play_numbers = card_line[1];
        let winning_play: Vec<&str> = just_winning_numbers_and_play_numbers.split("|").collect();
        let mut winning_numbers: Vec<i32> = Vec::new();
        let mut play_numbers: Vec<i32> = Vec::new();
        for number in winning_play[0].split_whitespace() {
            winning_numbers.push(number.parse::<i32>().unwrap());
        }
        for number in winning_play[1].split_whitespace() {
            play_numbers.push(number.parse::<i32>().unwrap());
        }
        Self {
            winning_numbers,
            play_numbers,
        }
    }
}

impl Scratchcard {
    fn score(&self) -> i32 {
        let mut matches: i32 = 0;
        for number in self.play_numbers.iter() {
            if self.winning_numbers.contains(number) {
                matches += 1;
            }
        }
        if matches > 0 {
            return 2i32.pow((matches - 1) as u32);
        } else {
            return 0;
        }
    }
}

fn load_input() -> Vec<Scratchcard> {
    let lines: Vec<String> = get_lines_from_filepath("data/day_four_input.txt");
    let mut scratchcards: Vec<Scratchcard> = Vec::new();
    for line in lines.iter() {
        scratchcards.push(Scratchcard::from_line(line));
    }
    scratchcards
}

fn part_one(scratchcards: &Vec<Scratchcard>) -> i32 {
    let mut score: i32 = 0;
    for scratchcard in scratchcards.iter() {
        println!("Scratchcard: {:?}", scratchcard);
        println!("Scratchcard score: {}", scratchcard.score());
        score += scratchcard.score();
    }
    println!("Part one score: {}", score);
    score
}

fn main() {
    let scratchcards: Vec<Scratchcard> = load_input();
    part_one(&scratchcards);
}
