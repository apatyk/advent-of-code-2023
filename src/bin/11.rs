advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let mut shortest_paths_sum: u32 = 0;

    // double all blank rows and columns

    // find coordinates of all galaxies

    // determine all shortest paths (N choose 2)

    // use the Lee algorithm for finding shortest path
    // https://stackoverflow.com/questions/2311486/how-to-calculate-the-shortest-path-between-two-points-in-a-grid

    // sum up shortest paths between each pair

    Some(shortest_paths_sum)
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
        assert_eq!(result.unwrap(), 374);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
