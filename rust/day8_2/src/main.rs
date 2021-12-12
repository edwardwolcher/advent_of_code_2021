#![allow(dead_code)]

const INPUT: &'static str = include_str!(
    "/Users/ew/Library/Mobile Documents/com~apple~CloudDocs/Projects/advent_2021/data/day8.txt"
);

// 1 =>       c,       f
// 7 => a,    c,       f
// 4 =>    b, c, d,    f
// 2 => a,    c, d, e,    g
// 3 => a,    c, d,    f, g
// 5 => a, b,    d,    f, g
// 6 => a, b,    d, e, f, g
// 0 => a, b, c,    e, f, g
// 9 => a, b, c, d,    f, g
// 8 => a, b, c, d, e, f, g
//      8  6  8  7  4  9  7

fn main() {
    let solution = process_input(INPUT);
    println!("Day 8_2 solution: {}", solution);
}

fn process_input(input: &str) -> u32 {
    let mut count = 0;
    for line in input.lines() {
        let (input, output) = line.split_once("|").unwrap();
        let input = input.split(' ');
        let one = input.clone().find(|i| i.len() == 2).unwrap();
        let four = input.clone().find(|i| i.len() == 4).unwrap();
        let output = output
            .split(' ')
            .skip(1)
            .map(|i| match i.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                len => match (
                    len,
                    i.chars().filter(|&j| one.contains(j)).count(),
                    i.chars().filter(|&j| four.contains(j)).count(),
                ) {
                    (5, 1, 3) => 5,
                    (5, 2, 3) => 3,
                    (5, _, 2) => 2,
                    (6, 1, _) => 6,
                    (6, _, 3) => 0,
                    (6, _, 4) => 9,
                    _ => unreachable!(),
                },
            })
            .enumerate()
            .fold(0, |sum, (i, n)| sum + n * 10_u32.pow(3 - i as u32));
        count += output;
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
    fn day8_2_test() {
        assert_eq!(61229, process_input(INPUT))
    }
}
