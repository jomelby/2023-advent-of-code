use advent2023::advent_utils::get_lines_from_filepath;

#[derive(Debug, Clone)]
pub struct Scratchcard {
    id: i32,
    winning_numbers: Vec<i32>,
    play_numbers: Vec<i32>,
}

impl Scratchcard {
    fn from_line(line: &String) -> Self {
        // strip card number from the line
        let card_line: Vec<&str> = line.split(":").collect();
        // split the winning numbers from the play numbers
        let card_number: i32 = card_line[0]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
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
            id: card_number,
            winning_numbers,
            play_numbers,
        }
    }
}

impl Scratchcard {
    fn score(&self) -> i32 {
        let matches: i32 = self.matches();
        if matches > 0 {
            return 2i32.pow((matches - 1) as u32);
        } else {
            return 0;
        }
    }
}

impl Scratchcard {
    fn matches(&self) -> i32 {
        let mut matches: i32 = 0;
        for number in self.play_numbers.iter() {
            if self.winning_numbers.contains(number) {
                matches += 1;
            }
        }
        matches
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

fn get_more_cards(
    starting_id: i32,
    number_of_cards: i32,
    scratchcards: &Vec<Scratchcard>,
) -> Vec<Scratchcard> {
    let new_scratchcards: Vec<Scratchcard> =
        scratchcards[starting_id as usize..(starting_id + number_of_cards) as usize].to_vec();
    new_scratchcards
}

fn play_card(input_score: i32, scratchcard: Scratchcard, scratchcards: &Vec<Scratchcard>) -> i32 {
    let mut running_score = input_score + 1;
    for extra_scratchcard in get_more_cards(scratchcard.id, scratchcard.matches(), scratchcards) {
        running_score = play_card(running_score, extra_scratchcard, scratchcards)
    }
    running_score
}

fn part_two(scratchcards: &Vec<Scratchcard>) -> i32 {
    let mut score: i32 = 0;
    for scratchcard in scratchcards.iter() {
        let scratchcard_copy = scratchcard.clone();
        score = play_card(score, scratchcard_copy, scratchcards)
    }
    println!("Part two score: {}", score);
    score
}

fn main() {
    let scratchcards: Vec<Scratchcard> = load_input();
    part_one(&scratchcards);
    part_two(&scratchcards);
}
