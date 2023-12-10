advent_of_code::solution!(4);

use std::collections::HashMap;

fn prepare_input(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<String> = prepare_input(input);
    let mut total_points: u32 = 0;

    for line in lines {
        let mut card_points: u32 = 0;
        let split_line: Vec<&str> = line.split(':').collect();

        let all_numbers: Vec<&str> = split_line[1].split(" | ").collect();
        let winning_numbers: Vec<u32> = all_numbers[0]
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let scratched_numbers: Vec<u32> = all_numbers[1]
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        for number in scratched_numbers {
            if winning_numbers.contains(&number) {
                if card_points == 0 {
                    card_points += 1;
                } else {
                    card_points *= 2;
                }
            };
        }

        total_points += card_points;
    }

    Some(total_points)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<String> = prepare_input(input);
    let mut card_copies: HashMap<u32, u32> = HashMap::new();

    // pre-populate all cards with 1 copy
    for line in &lines {
        let split_line: Vec<&str> = line.split(':').collect();
        let card: Vec<&str> = split_line[0].split_whitespace().collect();
        let card_number: u32 = card[1].parse().unwrap();

        card_copies.insert(card_number, 1);
    }

    // cycle through each line looking for matches and increasing copies accordingly
    for line in lines {
        let mut card_matches = 0;
        let split_line: Vec<&str> = line.split(':').collect();

        let card: Vec<&str> = split_line[0].split_whitespace().collect();
        let card_number: u32 = card[1].parse().unwrap();

        let all_numbers: Vec<&str> = split_line[1].split(" | ").collect();
        let winning_numbers: Vec<u32> = all_numbers[0]
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let scratched_numbers: Vec<u32> = all_numbers[1]
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        for number in scratched_numbers {
            if winning_numbers.contains(&number) {
                card_matches += 1;
            };
        }

        for m in 1..=card_matches {
            let copy_card_number = card_number + m;
            *card_copies.get_mut(&copy_card_number).unwrap() +=
                *card_copies.get(&card_number).unwrap();
        }
    }

    let total_scratchcards: u32 = card_copies.values().sum();

    Some(total_scratchcards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 13);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 30);
    }
}
