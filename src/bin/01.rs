advent_of_code::solution!(1);

use std::collections::HashMap;

fn prepare_input(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

// PART 1
pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    let lines = prepare_input(input);

    for line in lines {
        let mut calibration_value: String = String::new();

        // iterate forwards adding first found numeric character to running total
        for c in line.chars() {
            if c.is_numeric() {
                calibration_value.push(c);
                break;
            }
        }

        // iterate backwards adding first found numeric character to running total
        for c in line.chars().rev() {
            if c.is_numeric() {
                calibration_value.push(c);
                break;
            }
        }

        let line_total: u32 = calibration_value.parse().unwrap();
        total += line_total;
    }

    Some(total)
}

fn replace_with_mappings(input: String, mappings: &HashMap<&str, &str>) -> String {
    let mut result = input;

    // find all possible mappings in each line
    let mut possible_mappings: Vec<(usize, &str)> = Vec::new();
    for map_key in mappings.keys() {
        let matches: Vec<_> = result.match_indices(map_key).collect();
        for m in matches {
            possible_mappings.push((m.0, map_key));
        }
    }

    // replace first character in spelled out word with digit mapping to avoid
    // interfering with overlaps
    for mapping in possible_mappings {
        let index = mapping.0;
        let key = &mapping.1;
        result.replace_range(index..(index + 1), mappings[key]);
    }

    result.to_string()
}

// PART 2
pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    let mut mappings = HashMap::new();
    mappings.insert("one", "1");
    mappings.insert("two", "2");
    mappings.insert("three", "3");
    mappings.insert("four", "4");
    mappings.insert("five", "5");
    mappings.insert("six", "6");
    mappings.insert("seven", "7");
    mappings.insert("eight", "8");
    mappings.insert("nine", "9");

    let lines = prepare_input(input);

    for line in lines {
        let mut calibration_value: String = String::new();

        // map spelled out numbers to digits
        let mapped_line = replace_with_mappings(line.clone(), &mappings);

        // iterate forwards adding first found numeric character to running total
        for c in mapped_line.chars() {
            if c.is_numeric() {
                calibration_value.push(c);
                break;
            }
        }

        // iterate backwards adding first found numeric character to running total
        for c in mapped_line.chars().rev() {
            if c.is_numeric() {
                calibration_value.push(c);
                break;
            }
        }
        let line_total: u32 = calibration_value.parse().unwrap();
        total += line_total;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result.unwrap(), 142);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result.unwrap(), 332);
    }
}
