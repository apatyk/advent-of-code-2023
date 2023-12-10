advent_of_code::solution!(3);

use regex::Regex;
use std::collections::HashMap;

fn parse_special_characters(text: &str) -> Vec<char> {
    let re = Regex::new(r"[0-9.\n]").unwrap();
    let special_characters = re.replace_all(text, "").to_string();

    special_characters.chars().collect()
}

fn point_to_index(cartesian_point: (usize, usize), width: usize) -> usize {
    width * cartesian_point.0 + cartesian_point.1
}

fn create_bounding_box(
    text_array: &str,
    width: usize,
    start_index: usize,
    stop_index: usize,
    max_row: usize,
) -> (String, Vec<usize>) {
    // build bounding box
    let mut bounding_box = String::new();
    let mut bb_indices: Vec<usize> = vec![];

    // determine rows and columns using row major order and preventing overflow
    let row = start_index / width;
    let bb_first_col = if start_index % width == 0 {
        0
    } else {
        start_index % width - 1
    };
    let bb_last_col = if stop_index / width != row {
        width - 1
    } else {
        stop_index % width + 1
    };
    let bb_first_row = if row == 0 { 0 } else { row - 1 };
    let bb_last_row = if row == max_row - 1 {
        max_row - 1
    } else {
        row + 1
    };

    // set up top left, top right, bottom left, bottom right indices
    let bb_tl_index = point_to_index((bb_first_row, bb_first_col), width);
    let bb_tr_index = point_to_index((bb_first_row, bb_last_col), width);
    let bb_bl_index = point_to_index((bb_last_row, bb_first_col), width);
    let bb_br_index = point_to_index((bb_last_row, bb_last_col), width);

    let bb_top_edge = &text_array[bb_tl_index..bb_tr_index];
    let bb_bottom_edge = &text_array[bb_bl_index..bb_br_index];

    let left_edge_index = point_to_index((row, bb_first_col), width);
    let right_edge_index = point_to_index((row, bb_last_col - 1), width);

    let bb_left_edge = &text_array.chars().nth(left_edge_index).unwrap();
    let bb_right_edge = &text_array.chars().nth(right_edge_index).unwrap();

    bounding_box.push_str(bb_top_edge);
    bounding_box.push_str(&bb_left_edge.to_string());
    bounding_box.push_str(&bb_right_edge.to_string());
    bounding_box.push_str(bb_bottom_edge);

    bb_indices.extend(bb_tl_index..bb_tr_index);
    bb_indices.push(left_edge_index);
    bb_indices.push(right_edge_index);
    bb_indices.extend(bb_bl_index..bb_br_index);

    (bounding_box, bb_indices)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    let special_characters = parse_special_characters(input);

    // load input into 1D vector of characters, represent 2D in row major order
    let str_array = input.replace('\n', "");
    let char_array = str_array.chars();
    let width = input.find('\n').unwrap();
    let max_row = str_array.len() / width;

    let mut number_str = String::new();
    let mut number_start_index: usize = 0;
    for (index, item) in char_array.enumerate() {
        // build numbers as they are encountered on the same row
        if item.is_ascii_digit() && (index / width) == (number_start_index / width) {
            number_str.push(item);
        } else {
            // after number or row ends, convert to integer & check bounding box chars
            if !number_str.is_empty() {
                let number: u32 = number_str.parse().unwrap();

                // create bounding box of characters
                let (bounding_box, _) =
                    create_bounding_box(&str_array, width, number_start_index, index, max_row);

                // check bounding box for special characters (not '.' or digit)
                if bounding_box
                    .chars()
                    .any(|c| special_characters.contains(&c))
                {
                    total += number;
                }
            }
            // corner case for when row ends with a digit & next row starts with a digit
            if item.is_ascii_digit() {
                number_str = item.to_string();
            } else {
                number_str = String::from("");
            }
            number_start_index = index + 1;
        };
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    let gear_character = '*';

    let mut possible_gears: HashMap<usize, Vec<u32>> = HashMap::new();

    // load input into 1D vector of characters, represent 2D in row major order
    let str_array = input.replace('\n', "");
    let char_array = str_array.chars();
    let width = input.find('\n').unwrap();
    let max_row = str_array.len() / width;

    let mut number_str = String::new();
    let mut number_start_index: usize = 0;
    for (index, item) in char_array.enumerate() {
        // build numbers as they are encountered on the same row
        if item.is_ascii_digit() && (index / width) == (number_start_index / width) {
            number_str.push(item);
        } else {
            // after number or row ends, convert to integer & check bounding box chars
            if !number_str.is_empty() {
                let number: u32 = number_str.parse().unwrap();

                // create bounding box of characters
                let (bounding_box, bb_indices) =
                    create_bounding_box(&str_array, width, number_start_index, index, max_row);

                // check bounding box for special gear character
                let bb_gear_index = bounding_box.find(gear_character);
                if bb_gear_index.is_some() {
                    // add number to a hash map keyed on the index of the gear character
                    let gear_index = bb_indices[bb_gear_index.unwrap()];
                    possible_gears
                        .entry(gear_index)
                        .or_default()
                        .push(number);
                }
            }
            // corner case for when row ends with a digit & next row starts with a digit
            if item.is_ascii_digit() {
                number_str = item.to_string();
            } else {
                number_str = String::from("");
            }
            number_start_index = index + 1;
        };
    }

    for vec in possible_gears.values() {
        if vec.len() == 2 {
            total += vec[0] * vec[1];
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 4361);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 467835);
    }
}
