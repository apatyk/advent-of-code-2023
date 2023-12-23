advent_of_code::solution!(15);

use std::collections::HashMap;

fn prepare_input(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

fn calculate_hash(string: &str) -> u32 {
    let mut step_total: u32 = 0;
    let chars = string.chars();

    for c in chars {
        step_total += c as u32; // ASCII value

        step_total *= 17;
        step_total %= 256;
    }

    step_total
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut initialization_sequence_total: u32 = 0;
    let lines: Vec<String> = prepare_input(input);
    let steps: Vec<&str> = lines[0].split(',').collect();

    for step in steps {
        // calculate hash value of step and continue summing
        initialization_sequence_total += calculate_hash(step);
    }

    Some(initialization_sequence_total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_focusing_power: u32 = 0;
    let mut lens_boxes: HashMap<u32, Vec<(String, Option<u32>)>> = HashMap::new();

    let lines: Vec<String> = prepare_input(input);
    let steps: Vec<&str> = lines[0].split(',').collect();

    for step in steps {
        let mut label: &str;
        let mut focal_length: Option<u32> = None;

        // parse out label and focal length from step
        if step.find('=').is_some() {
            let split_step: Vec<&str> = step.split('=').collect();
            label = split_step[0];
            focal_length = Some(split_step[1].parse::<u32>().unwrap());
        } else {
            let split_step: Vec<&str> = step.split('-').collect();
            label = split_step[0];
        };

        // calculate hash value of label
        let hash_value: u32 = calculate_hash(label);

        if let Some(lens_vec) = lens_boxes.get_mut(&hash_value) {
            if focal_length.is_some() {
                // update focal length on lens already in box
                if let Some((_, focal_length_value)) =
                    lens_vec.iter_mut().find(|(s, _)| s == &label)
                {
                    *focal_length_value = focal_length;
                } else {
                    // add lens to hash map if a focal length is set
                    lens_boxes
                        .entry(hash_value)
                        .or_default()
                        .push((label.to_string(), focal_length));
                }
            } else {
                // remove lens from hash map if focal length is not set
                lens_vec.retain(|(s, _)| s != &label);
            };
        } else {
            lens_boxes
                .entry(hash_value)
                .or_default()
                .push((label.to_string(), focal_length));
        }
    }

    // calculate and sum focusing powers
    for (box_num, lenses) in &lens_boxes {
        for (index, lens) in lenses.iter().enumerate() {
            total_focusing_power += (box_num + 1) * (index as u32 + 1) * lens.1.unwrap_or(1);
        }
    }
    Some(total_focusing_power)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 1320);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 145);
    }
}
