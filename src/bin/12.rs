advent_of_code::solution!(12);

fn prepare_input(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

fn determine_possible_arrangements(springs: String, num_broken: Vec<u32>) -> u32 {
    let mut possible_arrangements: u32 = 0;

    possible_arrangements
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_arrangements: u32 = 0;
    let lines: Vec<String> = prepare_input(input);

    for line in lines {
        let split_line: Vec<&str> = line.split(' ').collect();
        let spring_conditions: String = split_line[0].to_string();
        let num_broken_springs: Vec<u32> = split_line[1]
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let possible_line_arrangements =
            determine_possible_arrangements(spring_conditions, num_broken_springs);

        total_arrangements += possible_line_arrangements;
    }

    Some(total_arrangements)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
