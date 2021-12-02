use std::fs;

fn main() {
    let data_path =
        "/Users/ew/Library/Mobile Documents/com~apple~CloudDocs/Projects/advent_2021/data/day1.txt";
    let input_list = read_input(data_path);
    let increases = count_increases(map_to_runs(input_list));
    print!("Day 1 part Two: {}\n", increases);
}

fn read_input(path: &str) -> Vec<usize> {
    let mut input_list: Vec<usize> = Vec::new();
    let contents = fs::read_to_string(path).expect("error reading file");
    for line in contents.lines() {
        let measurement = line.parse::<usize>().unwrap();
        input_list.push(measurement);
    }
    input_list
}

fn count_increases(input_list: Vec<usize>) -> usize {
    let mut increases: usize = 0;
    for (i, _measurement) in input_list.iter().enumerate() {
        if (i != 0) && (input_list[i] > input_list[i - 1]) {
            increases += 1;
        }
    }
    increases
}

fn map_to_runs(input_list: Vec<usize>) -> Vec<usize> {
    let mut runs = Vec::new();
    for (i, _measurement) in input_list.iter().enumerate() {
        if i + 2 >= input_list.len() {
            break;
        }
        let run = input_list[i] + input_list[i + 1] + input_list[i + 2];
        runs.push(run);
    }
    runs
}

#[cfg(test)]
mod tests {
    use crate::{count_increases, map_to_runs};

    #[test]
    fn day1_2_test() {
        let test_input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increases(map_to_runs(test_input)), 5);
    }
}
