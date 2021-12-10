use std::fs;

pub const DATA_PATH: &str =
    "/Users/ew/Library/Mobile Documents/com~apple~CloudDocs/Projects/advent_2021/data/day6.txt";

fn main() {
    let input_string = fs::read_to_string(DATA_PATH).expect("error reading data");
    let mut population = parse_input(input_string);
    for _ in 0..80 {
        iterate(&mut population);
    }
    let pop_sum: usize = population.iter().sum();

    println!("Solution: {}", pop_sum);
}

fn parse_input(input: String) -> Vec<usize> {
    let mut gen_list = vec![0; 9];
    input
        .split(",")
        .for_each(|i| gen_list[i.parse::<usize>().unwrap()] += 1);
    gen_list
}

fn iterate(population: &mut Vec<usize>) {
    let parents = population[0];
    for i in 0..8 {
        population[i] = population[i + 1];
    }
    population[8] = parents;
    population[6] += parents;
}

#[cfg(test)]
mod tests {
    use crate::iterate;

    use crate::parse_input;

    #[test]
    fn day6_1_test() {
        let test_input = String::from("3,4,3,1,2");

        let mut population1 = parse_input(test_input.clone());
        for _ in 0..18 {
            iterate(&mut population1);
        }
        let pop_sum: usize = population1.iter().sum();

        assert_eq!(pop_sum, 26);

        let mut population2 = parse_input(test_input);
        for _ in 0..80 {
            iterate(&mut population2);
        }
        let pop_sum: usize = population2.iter().sum();

        assert_eq!(pop_sum, 5934)
    }
}
