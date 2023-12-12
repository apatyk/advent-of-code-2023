advent_of_code::solution!(8);

use std::collections::HashMap;

fn prepare_input(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

fn vec_lcm(vec: Vec<u32>) -> u32 {
    let mut lcm = vec[0];

    for &n in &vec[1..] {
        lcm = (n * lcm) / (gcd(n, lcm));
    }

    lcm
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_steps: u32 = 0;
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    let lines: Vec<String> = prepare_input(input);

    // parse directions (sequence of L or R) from input
    let directions = lines[0].to_owned();

    // parse nodes from input and add to a hash map keyed on the node name
    for line in &lines[2..] {
        let split_line: Vec<&str> = line.split(" = ").collect();

        let node: &str = split_line[0];
        let downstream_nodes: Vec<&str> = split_line[1]
            .split(", ")
            .map(|node| node.trim_matches(|c| c == '(' || c == ')'))
            .collect();
        let left: &str = downstream_nodes[0];
        let right: &str = downstream_nodes[1];

        nodes.insert(node.to_owned(), (left.to_owned(), right.to_owned()));
    }

    // traverse nodes
    let mut current_node = String::from("AAA");
    let mut dir_index: usize = 0;
    while current_node != *"ZZZ" {
        let direction: char = directions.chars().nth(dir_index).unwrap();

        let downstream_nodes = nodes.get(&current_node).unwrap();
        current_node = match direction {
            'L' => downstream_nodes.0.to_owned(),
            'R' => downstream_nodes.1.to_owned(),
            _ => current_node,
        };
        dir_index = (dir_index + 1) % directions.len();
        total_steps += 1;
    }

    Some(total_steps)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    let lines: Vec<String> = prepare_input(input);

    // parse directions (sequence of L or R) from input
    let directions = lines[0].to_owned();

    // parse nodes from input and add to a hash map keyed on the node name
    for line in &lines[2..] {
        let split_line: Vec<&str> = line.split(" = ").collect();

        let node: &str = split_line[0];
        let downstream_nodes: Vec<&str> = split_line[1]
            .split(", ")
            .map(|node| node.trim_matches(|c| c == '(' || c == ')'))
            .collect();
        let left: &str = downstream_nodes[0];
        let right: &str = downstream_nodes[1];

        nodes.insert(node.to_owned(), (left.to_owned(), right.to_owned()));
    }

    // traverse nodes
    // start with all nodes ending with "A"
    let starting_nodes: Vec<String> = nodes
        .keys()
        .filter(|&s| s.ends_with('A'))
        .map(String::from)
        .collect();
    // traversing until all current nodes end with "Z" is too computationally expensive
    // determine individual loop lengths and calculate LCM
    let mut all_node_cycle_lengths: Vec<u32> = vec![];
    for node in starting_nodes {
        let mut dir_index: usize = 0;
        let mut steps = 0;
        let mut current_node = node;
        while !current_node.ends_with('Z') {
            let direction: char = directions.chars().nth(dir_index).unwrap();

            let downstream_nodes = nodes.get(&current_node).unwrap();
            current_node = match direction {
                'L' => downstream_nodes.0.to_owned(),
                'R' => downstream_nodes.1.to_owned(),
                _ => current_node,
            };
            dir_index = (dir_index + 1) % directions.len();
            steps += 1;
        }
        all_node_cycle_lengths.push(steps);
    }

    all_node_cycle_lengths.sort();
    let total_steps = vec_lcm(all_node_cycle_lengths);

    Some(total_steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result.unwrap(), 6);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result.unwrap(), 6);
    }
}
