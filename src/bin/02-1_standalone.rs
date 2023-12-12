use std::env;
use std::fs::read_to_string;
use std::path::Path;

static MAX_RED: u16 = 12;
static MAX_GREEN: u16 = 13;
static MAX_BLUE: u16 = 14;

fn read_lines(filename: &Path) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    // use first command line argument as input file to parse
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);

    let mut game_ids_total: u16 = 0;

    // parse out information from each line
    let lines = read_lines(path);
    for line in lines {
        let mut possible: bool = true;
        let split_line: Vec<&str> = line.split(':').collect();
        let game: Vec<&str> = split_line[0].split_whitespace().collect();
        let cube_handfuls: Vec<&str> = split_line[1].split("; ").collect();

        let game_id: u16 = game[1].parse().unwrap();

        for cube_handful in cube_handfuls {
            let cube_draws: Vec<&str> = cube_handful.split(", ").collect();
            for cube_draw in cube_draws {
                let split_cube_draw: Vec<&str> = cube_draw.split_whitespace().collect();

                let cube_number: u16 = split_cube_draw[0].parse().unwrap();
                let cube_color: &str = split_cube_draw[1];

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

    println!("Total possible game IDs: {}", game_ids_total);
}
