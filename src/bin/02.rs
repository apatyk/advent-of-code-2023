advent_of_code::solution!(2);

use std::collections::HashMap;

static MAX_RED: u32 = 12;
static MAX_GREEN: u32 = 13;
static MAX_BLUE: u32 = 14;

fn prepare_input(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

// PART 1
pub fn part_one(input: &str) -> Option<u32> {
    let mut game_ids_total: u32 = 0;
    let lines = prepare_input(input);

    // parse out information from each line
    for line in lines {
        let mut possible: bool = true;
        let split_line: Vec<&str> = line.split(':').collect();

        let game: Vec<&str> = split_line[0].split_whitespace().collect();
        let cube_handfuls: Vec<&str> = split_line[1].split("; ").collect();

        let game_id: u32 = game[1].parse().unwrap();

        for cube_handful in cube_handfuls {
            let cube_draws: Vec<&str> = cube_handful.split(", ").collect();
            for cube_draw in cube_draws {
                let split_cube_draw: Vec<&str> = cube_draw.split_whitespace().collect();

                let cube_number: u32 = split_cube_draw[0].parse().unwrap();
                let cube_color: &str = split_cube_draw[1];

                // determine if each combination is possible
                possible = match cube_color {
                    "red" => {
                        if cube_number > MAX_RED {
                            false
                        } else {
                            possible
                        }
                    }
                    "green" => {
                        if cube_number > MAX_GREEN {
                            false
                        } else {
                            possible
                        }
                    }
                    "blue" => {
                        if cube_number > MAX_BLUE {
                            false
                        } else {
                            possible
                        }
                    }
                    _ => true,
                };
            }
        }
        if possible {
            game_ids_total += game_id;
        };
    }
    Some(game_ids_total)
}

// PART 2
pub fn part_two(input: &str) -> Option<u32> {
    let mut cube_power_total: u32 = 0;

    let lines = prepare_input(input);
    for line in lines {
        let mut cube_power: u32 = 1;

        // initialize hash map to store minimum count for each color
        let mut cube_counts: HashMap<&str, u32> = HashMap::new();
        cube_counts.insert("red", 0);
        cube_counts.insert("green", 0);
        cube_counts.insert("blue", 0);

        // parse out information from each line
        let split_line: Vec<&str> = line.split(':').collect();
        let cube_handfuls: Vec<&str> = split_line[1].split("; ").collect();
        for cube_handful in cube_handfuls {
            let cube_draws: Vec<&str> = cube_handful.split(", ").collect();
            for cube_draw in cube_draws {
                let split_cube_draw: Vec<&str> = cube_draw.split_whitespace().collect();

                let cube_number: u32 = split_cube_draw[0].parse().unwrap();
                let cube_color: &str = split_cube_draw[1];

                // determine if number of cubes is new minimum for color (max encountered)
                if cube_number > *cube_counts.get_mut(cube_color).unwrap() {
                    *cube_counts.get_mut(cube_color).unwrap() = cube_number;
                };
            }
        }
        // calculate power of the minimum set of cubes
        for value in cube_counts.values() {
            cube_power *= value;
        }
        cube_power_total += cube_power;
    }

    Some(cube_power_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 8);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 2286);
    }
}
