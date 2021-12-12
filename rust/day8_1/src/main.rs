#![allow(dead_code)]

const INPUT: &'static str = include_str!(
    "/Users/ew/Library/Mobile Documents/com~apple~CloudDocs/Projects/advent_2021/data/day8.txt"
);

fn main() {
    let solution = process_input(INPUT);
    println!("Day 8_1 solution: {}", solution);
}

fn process_input(input: &str) -> usize {
    let mut count = 0;
    for line in input.lines() {
        let (_, outputs) = line.split_once("|").unwrap();
        for code in outputs.split(' ') {
            match code.len() {
                2 => count += 1,
                3 => count += 1,
                4 => count += 1,
                7 => count += 1,
                _ => (),
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::process_input;

    const INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn day8_1_test() {
        assert_eq!(26, process_input(INPUT))
    }
}
