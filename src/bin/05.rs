advent_of_code::solution!(5);

use std::fmt::Debug;

#[derive(Debug)]
struct SeedSet {
    min: u32,
    max: u32,
}

#[derive(Debug)]
struct Map {
    _name: String,
    src_ranges: Vec<(u32, u32)>,
    dst_ranges: Vec<(u32, u32)>,
}

fn prepare_input(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

fn map_value(input: u32, map: &Map) -> u32 {
    let mut output: u32 = input;

    for (index, range) in map.src_ranges.iter().enumerate() {
        if input >= range.0 && input < range.1 {
            output = map.dst_ranges[index].0 + input - range.0;
            break;
        };
    }

    output
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut min_location = u32::MAX;
    let mut maps: Vec<Map> = vec![];

    let lines = prepare_input(input);

    // get seed numbers from input
    let split_first_line: Vec<&str> = lines[0].split(':').collect();
    let seeds: Vec<u32> = split_first_line[1]
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    // build maps from input
    for line in lines {
        if line.is_empty() || line.contains("seeds") {
            continue;
        } else if line.contains("map") {
            let split_line: Vec<&str> = line.split_whitespace().collect();
            maps.push(Map {
                _name: split_line[0].to_string(),
                src_ranges: vec![],
                dst_ranges: vec![],
            })
        } else {
            let map_values: Vec<u32> = line
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            let dst_start = map_values[0];
            let src_start = map_values[1];
            let size = map_values[2];
            let last_index = maps.len() - 1;

            maps[last_index]
                .src_ranges
                .push((src_start, src_start + size));
            maps[last_index]
                .dst_ranges
                .push((dst_start, dst_start + size));
        }
    }

    // determine mapping for each seed
    for seed in seeds {
        let mut tmp: u32 = map_value(seed, &maps[0]);
        for map in &maps[1..] {
            tmp = map_value(tmp, map);
        }

        if tmp < min_location {
            min_location = tmp;
        }
    }

    Some(min_location)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut min_location = u32::MAX;
    let mut seeds: Vec<SeedSet> = vec![];
    let mut maps: Vec<Map> = vec![];

    let lines = prepare_input(input);

    // get seed numbers from input
    let split_first_line: Vec<&str> = lines[0].split(':').collect();
    let seed_numbers: Vec<u32> = split_first_line[1]
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut all_seed_numbers: Vec<(u32, u32)> = vec![];
    for n in 0..seed_numbers.len() / 2 {
        let start: u32 = seed_numbers[2 * n];
        let length: u32 = seed_numbers[2 * n + 1];
        all_seed_numbers.push((start, start + length));
    }

    for (start, end) in all_seed_numbers {
        seeds.push(SeedSet {
            min: start,
            max: end,
        });
    }

    // build maps from input
    for line in lines {
        if line.is_empty() || line.contains("seeds") {
            continue;
        } else if line.contains("map") {
            let split_line: Vec<&str> = line.split_whitespace().collect();
            maps.push(Map {
                _name: split_line[0].to_string(),
                src_ranges: vec![],
                dst_ranges: vec![],
            })
        } else {
            let map_values: Vec<u32> = line
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            let dst_start = map_values[0];
            let src_start = map_values[1];
            let size = map_values[2];
            let last_index = maps.len() - 1;

            maps[last_index]
                .src_ranges
                .push((src_start, src_start + size));
            maps[last_index]
                .dst_ranges
                .push((dst_start, dst_start + size));
        }
    }

    // determine mapping for each seed
    for seed in seeds {
        let mut min_seed_location: u32 = u32::MAX;
        for seed_number in seed.min..seed.max {
            let mut tmp: u32 = map_value(seed_number, &maps[0]);
            for map in &maps[1..] {
                tmp = map_value(tmp, map);
            }
            if tmp < min_seed_location {
                min_seed_location = tmp
            }
        }

        if min_seed_location < min_location {
            min_location = min_seed_location;
        }
    }

    Some(min_location)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 35);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 46);
    }
}
