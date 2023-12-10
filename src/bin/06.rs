advent_of_code::solution!(6);

fn prepare_input(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut ways_to_beat_record: Vec<u32> = vec![];

    // parse times and distances for each race from input
    let lines = prepare_input(input);
    let split_first_line: Vec<&str> = lines[0].split(':').collect();
    let split_second_line: Vec<&str> = lines[1].split(':').collect();
    let times: Vec<u32> = split_first_line[1]
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let distances: Vec<u32> = split_second_line[1]
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let races: Vec<(u32, u32)> = times.into_iter().zip(distances.into_iter()).collect();

    for (time, record_distance) in races {
        let mut num_winning_races: u32 = 0;

        // iterate through possible button hold times
        // ignore min/max cases where boat moves 0 mm
        for hold_time in 1..time {
            let speed = hold_time;
            let distance = speed * (time - hold_time);
            if distance > record_distance {
                num_winning_races += 1;
            }
        }
        ways_to_beat_record.push(num_winning_races);
    }

    // calculate product of the number of ways to win each race
    let total: u32 = ways_to_beat_record
        .iter()
        .copied()
        .reduce(|a, b| a * b)
        .unwrap();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_ways_to_beat_record: u32 = 0;

    // parse times and distances for each race from input
    let lines = prepare_input(input);
    let split_first_line: Vec<&str> = lines[0].split(':').collect();
    let split_second_line: Vec<&str> = lines[1].split(':').collect();
    let race_time: u64 = split_first_line[1].replace(" ", "").parse().unwrap();

    let race_record_distance: u64 = split_second_line[1].replace(" ", "").parse().unwrap();

    // iterate through possible button hold times
    // ignore min/max cases where boat moves 0 mm
    for hold_time in 1..race_time {
        let speed: u64 = hold_time;
        let distance: u64 = speed * (race_time - hold_time);
        if distance > race_record_distance {
            total_ways_to_beat_record += 1;
        }
    }

    Some(total_ways_to_beat_record)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 288);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 71503);
    }
}
