use std::fs;

pub const DATA_PATH: &str =
    "/Users/ew/Library/Mobile Documents/com~apple~CloudDocs/Projects/advent_2021/data/day7.txt";

fn main() {
    let input_string = fs::read_to_string(DATA_PATH).expect("error reading data");
    let positions = parse_input(input_string);
    println!("{:?}", positions);
    let solution = get_solution(positions);
    println!("Day 7_1 Solution: {}", solution);
}

fn parse_input(input: String) -> Vec<i32> {
    input
        .split(",")
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}

fn get_solution(mut positions: Vec<i32>) -> i32 {
    positions.sort();
    let median = positions[positions.len() / 2];
    positions
        .iter()
        .fold(0, |acc, pos| acc + (median - pos).abs())
}

#[cfg(test)]
mod tests {
    use crate::{get_solution, parse_input};

    #[test]
    fn day7_1_test() {
        let test_input = String::from("16,1,2,0,4,2,7,1,2,14");
        let positions = parse_input(test_input);
        assert_eq!(37, get_solution(positions))
    }
}
