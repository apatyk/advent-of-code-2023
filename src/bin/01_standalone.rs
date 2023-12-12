use std::fs::read_to_string;

use std::collections::HashMap;
use std::path::Path;

static COUNT_SPELLED_OUT: bool = false;
static DEBUG: bool = false;

fn read_lines(filename: &Path) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
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

fn main() {
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

    let path = Path::new("calibration.txt");

    let mut total = 0;

    // read file line by line
    let lines = read_lines(path);
    for line in lines {
        let mut calibration_value: String = String::new();

        // map spelled out numbers to digits if needed
        let mapped_line = if COUNT_SPELLED_OUT {
            replace_with_mappings(line.clone(), &mappings)
        } else {
            line.clone()
        };

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
        let line_total: i32 = calibration_value.parse().unwrap();
        total += line_total;
        if DEBUG {
            println!("{} -> {}: {}", line, mapped_line, line_total);
        };
    }

    println!("{}", total);
}
