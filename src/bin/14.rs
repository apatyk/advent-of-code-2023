advent_of_code::solution!(14);

fn prepare_input(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_load: u32 = 0;

    let str_array: String = input.replace('\n', "");
    let original_platform = str_array.chars();
    let width: usize = input.find('\n').unwrap();
    let max_row = str_array.len() / width;

    // tilt each row

    Some(total_load)
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
        assert_eq!(result.unwrap(), 136);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
