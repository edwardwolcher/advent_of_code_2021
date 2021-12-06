use std::fs;

pub const DATA_PATH: &str =
    "/Users/ew/Library/Mobile Documents/com~apple~CloudDocs/Projects/advent_2021/data/day3.txt";
fn main() {
    let diagnostic_report = fs::read_to_string(DATA_PATH).expect("error reading data");
    let result = binary_diagnostic(diagnostic_report);
    println!("Day 3_1: {}", result)
}

fn binary_diagnostic(report: String) -> usize {
    let num_bits = report.lines().next().unwrap().len();

    let report_lines: Vec<&str> = report.lines().collect();
    let mut report_columns: Vec<String> = Vec::with_capacity(num_bits);

    for bit in 0..num_bits {
        let mut column = String::with_capacity(report_lines.len());
        for line in &report_lines {
            column.push(line.chars().nth(bit).unwrap());
        }
        report_columns.push(column);
    }

    let mut gamma_string = String::with_capacity(num_bits);
    let mut epsilon_string = String::with_capacity(num_bits);

    for column in report_columns {
        let (ones, zeroes) = column.chars().fold((0, 0), |(o, z), char| match char {
            '0' => (o + 1, z),
            '1' => (o, z + 1),
            _ => (o, z),
        });
        if ones > zeroes {
            gamma_string.push('1');
            epsilon_string.push('0');
        } else {
            gamma_string.push('0');
            epsilon_string.push('1');
        }
    }

    let gamma = usize::from_str_radix(&gamma_string[..], 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon_string[..], 2).unwrap();

    gamma * epsilon
}

#[cfg(test)]
mod tests {
    use crate::binary_diagnostic;

    #[test]
    fn day3_1_test() {
        let test_data = String::from(
            "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
        );

        assert_eq!(198, binary_diagnostic(test_data));
    }
}
