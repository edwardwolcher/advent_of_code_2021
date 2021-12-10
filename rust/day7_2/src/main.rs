use std::fs;

pub const DATA_PATH: &str =
    "/Users/ew/Library/Mobile Documents/com~apple~CloudDocs/Projects/advent_2021/data/day7.txt";

fn main() {
    let input_string = fs::read_to_string(DATA_PATH).expect("error reading data");
    let positions = parse_input(input_string);
    let solution = get_solution(positions);
    println!("Day 7_2 Solution: {}", solution);
}

fn parse_input(input: String) -> Vec<i32> {
    input
        .split(",")
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}

fn get_solution(positions: Vec<i32>) -> i32 {
    let mean = (positions.iter().sum::<i32>() as f64 / positions.len() as f64).round() as i32;
    let tries = Vec::from([mean - 1, mean, mean + 1]);

    tries
        .iter()
        .map(|t| get_costs(&positions, *t))
        .min()
        .unwrap()
}

fn get_costs(positions: &Vec<i32>, point: i32) -> i32 {
    positions.iter().fold(0, |acc, pos| {
        let distance = (pos - point).abs();
        let cost = (distance * (distance + 1)) / 2;
        println!("Distance: {}, Cost: {}", distance, cost);
        acc + cost
    })
}

#[cfg(test)]
mod tests {
    use crate::{get_solution, parse_input};

    #[test]
    fn day7_2_test() {
        let test_input = String::from("16,1,2,0,4,2,7,1,2,14");
        let positions = parse_input(test_input);
        assert_eq!(168, get_solution(positions))
    }
}
