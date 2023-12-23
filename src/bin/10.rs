advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let str_array: String = input.replace('\n', "");
    let char_array = str_array.chars();
    let width: usize = input.find('\n').unwrap();
    let max_row = str_array.len() / width;

    // Invert map so all non-pipes are blocked
    // Use the Lee algorithm
    // https://stackoverflow.com/questions/2288830/change-floodfill-algorithm-to-get-voronoi-territory-for-two-data-points/2288898#2288898

    // find location of 'S' for starting position
    let start_index: usize = char_array.clone().position(|c| c == 'S').unwrap();

    dbg!(start_index);
    dbg!(char_array.nth(0).unwrap());
    dbg!(char_array.clone().nth(start_index).unwrap());

    // move out in each direction
    dbg!(start_index - width);

    // move up one row
    dbg!(char_array.clone().nth(start_index - width).unwrap());
    // move down one row
    dbg!(char_array.clone().nth(start_index + width).unwrap());
    // move left one column
    dbg!(char_array.clone().nth(start_index - 1));
    // move right one column
    dbg!(char_array.clone().nth(start_index + 1));

    Some(0)
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
        assert_eq!(result.unwrap(), 8);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
