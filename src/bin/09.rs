advent_of_code::solution!(9);

fn prepare_input(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut predictions: Vec<i32> = vec![];
    let lines: Vec<String> = prepare_input(input);

    for line in lines {
        let mut last_values: Vec<i32> = vec![];

        // parse sensor readings from lines as unsigned integers
        let sensor_readings: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        last_values.push(sensor_readings[sensor_readings.len() - 1]);

        // calculate differences between array elements in 2-element windows
        let mut differences: Vec<i32> = sensor_readings.windows(2).map(|r| r[1] - r[0]).collect();
        // save last value of each for final computation
        while differences.iter().any(|&n| n != 0) {
            last_values.push(differences[differences.len() - 1]);
            differences = differences.windows(2).map(|r| r[1] - r[0]).collect();
        }

        // calculate prediction as sum of all last values
        predictions.push(last_values.iter().sum());
    }

    let total: i32 = predictions.iter().sum();

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut predictions: Vec<i32> = vec![];
    let lines: Vec<String> = prepare_input(input);

    for line in lines {
        let mut last_values: Vec<i32> = vec![];

        // parse sensor readings from lines as unsigned integers
        let sensor_readings: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .rev()
            .collect();
        last_values.push(sensor_readings[sensor_readings.len() - 1]);

        // calculate differences between array elements in 2-element windows
        let mut differences: Vec<i32> = sensor_readings.windows(2).map(|r| r[0] - r[1]).collect();
        last_values.push(differences[differences.len() - 1]);
        // save last value of each for final computation
        while differences.iter().any(|&n| n != 0) {
            differences = differences.windows(2).map(|r| r[0] - r[1]).collect();
            last_values.push(differences[differences.len() - 1]);
        }

        // extrapolate back one more step
        let mut prediction: i32 = last_values.pop().unwrap();
        for value in last_values.iter().rev() {
            prediction = value - prediction;
        }

        // calculate prediction as sum of all last values
        predictions.push(prediction);
    }

    let total: i32 = predictions.iter().sum();

    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 114);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 2);
    }
}
