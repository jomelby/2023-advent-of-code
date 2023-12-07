use advent2023::advent_utils::get_lines_from_filepath;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct AlmanacMap {
    destination_range_starts: Vec<i64>,
    source_range_starts: Vec<i64>,
    lengths: Vec<i64>,
}

impl AlmanacMap {
    fn from_lines(lines: &[String], map_name: &str) -> Self {
        let mut destination_range_starts: Vec<i64> = Vec::new();
        let mut source_range_starts: Vec<i64> = Vec::new();
        let mut lengths: Vec<i64> = Vec::new();
        let mut parse_section: bool = false;
        for line in lines.iter() {
            if line.starts_with(map_name) {
                parse_section = true;
            } else if parse_section && !line.is_empty() {
                let split_line = line.split_whitespace().collect::<Vec<&str>>();
                destination_range_starts.push(split_line[0].parse::<i64>().unwrap());
                source_range_starts.push(split_line[1].parse::<i64>().unwrap());
                lengths.push(split_line[2].parse::<i64>().unwrap());
            } else if !parse_section {
                continue;
            } else {
                break;
            }
        }
        Self {
            destination_range_starts,
            source_range_starts,
            lengths,
        }
    }
}

impl AlmanacMap {
    fn map(&self, source: i64) -> i64 {
        let mut destination: i64 = source;
        for index in 0..self.source_range_starts.len() {
            let source_range: std::ops::Range<i64> = self.source_range_starts[index]
                ..self.source_range_starts[index] + self.lengths[index];
            if source_range.contains(&source) {
                destination =
                    source - self.source_range_starts[index] + self.destination_range_starts[index];
                break;
            }
        }
        destination
    }
}

fn part_one(lines: &[String]) {
    let seeds: Vec<i64> = get_seeds(&lines);
    let seed_soil_map: AlmanacMap = AlmanacMap::from_lines(&lines, "seed-to-soil map:");
    let soil_fertilizer_map: AlmanacMap = AlmanacMap::from_lines(&lines, "soil-to-fertilizer map:");
    let fertilizer_water_map: AlmanacMap =
        AlmanacMap::from_lines(&lines, "fertilizer-to-water map:");
    let water_light_map: AlmanacMap = AlmanacMap::from_lines(&lines, "water-to-light map:");
    let light_temperature_map: AlmanacMap =
        AlmanacMap::from_lines(&lines, "light-to-temperature map:");
    let temperature_humidity_map: AlmanacMap =
        AlmanacMap::from_lines(&lines, "temperature-to-humidity map:");
    let humidity_location_map: AlmanacMap =
        AlmanacMap::from_lines(&lines, "humidity-to-location map:");
    let locations: Vec<i64> = seeds
        .par_iter()
        .map(|seed| {
            let soil = seed_soil_map.map(*seed);
            let fertilizer = soil_fertilizer_map.map(soil);
            let water = fertilizer_water_map.map(fertilizer);
            let light = water_light_map.map(water);
            let temperature = light_temperature_map.map(light);
            let humidity = temperature_humidity_map.map(temperature);
            let location = humidity_location_map.map(humidity);
            location
        })
        .collect();
    let min_location: Option<&i64> = locations.iter().min();
    println!("Min location: {}", min_location.unwrap());
}

fn part_two(lines: &[String]) {
    let seeds: Vec<i64> = get_seeds_range(&lines);
    let seed_soil_map: AlmanacMap = AlmanacMap::from_lines(&lines, "seed-to-soil map:");
    let soil_fertilizer_map: AlmanacMap = AlmanacMap::from_lines(&lines, "soil-to-fertilizer map:");
    let fertilizer_water_map: AlmanacMap =
        AlmanacMap::from_lines(&lines, "fertilizer-to-water map:");
    let water_light_map: AlmanacMap = AlmanacMap::from_lines(&lines, "water-to-light map:");
    let light_temperature_map: AlmanacMap =
        AlmanacMap::from_lines(&lines, "light-to-temperature map:");
    let temperature_humidity_map: AlmanacMap =
        AlmanacMap::from_lines(&lines, "temperature-to-humidity map:");
    let humidity_location_map: AlmanacMap =
        AlmanacMap::from_lines(&lines, "humidity-to-location map:");
    let locations: Vec<i64> = seeds
        .iter()
        .map(|seed| {
            let soil = seed_soil_map.map(*seed);
            let fertilizer = soil_fertilizer_map.map(soil);
            let water = fertilizer_water_map.map(fertilizer);
            let light = water_light_map.map(water);
            let temperature = light_temperature_map.map(light);
            let humidity = temperature_humidity_map.map(temperature);
            let location = humidity_location_map.map(humidity);
            location
        })
        .collect();
    let min_location: Option<&i64> = locations.iter().min();
    println!("Min location: {:?}", min_location.unwrap());
}

fn get_seeds(lines: &[String]) -> Vec<i64> {
    let mut seeds: Vec<i64> = Vec::new();
    for line in lines.iter() {
        if line.starts_with("seeds:") {
            let seed_line = line.split("seeds:").collect::<Vec<&str>>()[1];
            let seed_line = seed_line.split_whitespace().collect::<Vec<&str>>();
            for seed in seed_line.iter() {
                seeds.push(seed.parse::<i64>().unwrap());
            }
        }
    }
    seeds
}

fn get_seeds_range(lines: &[String]) -> Vec<i64> {
    let mut seeds: Vec<i64> = Vec::new();
    for line in lines.iter() {
        if line.starts_with("seeds:") {
            let seed_line = line.split("seeds:").collect::<Vec<&str>>()[1];
            let seed_line = seed_line.split_whitespace().collect::<Vec<&str>>();
            for index in (0..seed_line.len()).step_by(2) {
                let seed_start = seed_line[index].parse::<i64>().unwrap();
                let seed_range_length = seed_line[index + 1].parse::<i64>().unwrap();
                for seed in seed_start..seed_start + seed_range_length {
                    seeds.push(seed);
                }
            }
        }
    }
    seeds
}

fn main() {
    let lines: Vec<String> = get_lines_from_filepath("data/day_five_input.txt");
    part_one(&lines);
    part_two(&lines);
}
