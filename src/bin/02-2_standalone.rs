use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

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

    let mut cube_power_total: u32 = 0;

    // parse out information from each line
    let lines = read_lines(path);
    for line in lines {
        let mut cube_power: u32 = 1;

        let mut cube_counts: HashMap<&str, u32> = HashMap::new();
        cube_counts.insert("red", 0);
        cube_counts.insert("green", 0);
        cube_counts.insert("blue", 0);

        let split_line: Vec<&str> = line.split(':').collect();
        let cube_handfuls: Vec<&str> = split_line[1].split("; ").collect();

        for cube_handful in cube_handfuls {
            let cube_draws: Vec<&str> = cube_handful.split(", ").collect();
            for cube_draw in cube_draws {
                let split_cube_draw: Vec<&str> = cube_draw.split_whitespace().collect();

                let cube_number: u32 = split_cube_draw[0].parse().unwrap();
                let cube_color: &str = split_cube_draw[1];

                if cube_number > *cube_counts.get_mut(cube_color).unwrap() {
                    *cube_counts.get_mut(cube_color).unwrap() = cube_number;
                };
            }
        }
        for (_, value) in &cube_counts {
            cube_power *= value;
        }
        cube_power_total += cube_power;
    }

    println!("Sum of the power of cube sets: {}", cube_power_total);
}
