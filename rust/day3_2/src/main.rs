use std::fs;

pub const DATA_PATH: &str =
    "/Users/ew/Library/Mobile Documents/com~apple~CloudDocs/Projects/advent_2021/data/day3.txt";
fn main() {
    let diagnostic_report = fs::read_to_string(DATA_PATH).expect("error reading data");
    let result = life_support_rating(diagnostic_report);
    println!("Day 3_2: {}", result)
}

fn chars_at_idx(ratings: &Vec<&str>, idx: usize) -> (usize, usize) {
    let zeros: usize = ratings.iter().fold(0, |acc, line| {
        if line.chars().nth(idx).unwrap() == '0' {
            return acc + 1;
        } else {
            return acc;
        }
    });
    let ones: usize = ratings.len() - zeros;
    (zeros, ones)
}

fn life_support_rating(report: String) -> usize {
    let num_bits = report.lines().next().unwrap().len();
    let report_lines: Vec<&str> = report.lines().collect();
    let mut oxygen_ratings = report_lines.clone();
    let mut co2_ratings = report_lines.clone();

    let mut oxygen_rating: &str = "";
    let mut co2_rating: &str = "";

    for i in 0..num_bits {
        let (oxygen_zeroes, oxygen_ones) = chars_at_idx(&oxygen_ratings, i);
        if oxygen_zeroes > oxygen_ones {
            oxygen_ratings.retain(|rating| rating.chars().nth(i).unwrap() == '0');
        } else {
            oxygen_ratings.retain(|rating| rating.chars().nth(i).unwrap() == '1');
        }
        if oxygen_ratings.len() == 1 {
            oxygen_rating = oxygen_ratings[0];
            break;
        }
    }
    for i in 0..num_bits {
        let (co2_zeros, co2_ones) = chars_at_idx(&co2_ratings, i);
        if co2_zeros > co2_ones {
            co2_ratings.retain(|rating| rating.chars().nth(i).unwrap() == '1');
        } else {
            co2_ratings.retain(|rating| rating.chars().nth(i).unwrap() == '0');
        }
        if co2_ratings.len() == 1 {
            co2_rating = co2_ratings[0];
            break;
        }
    }

    let oxygen_rating_number = usize::from_str_radix(oxygen_rating, 2).unwrap();
    let co2_rating_number = usize::from_str_radix(co2_rating, 2).unwrap();

    oxygen_rating_number * co2_rating_number
}

#[cfg(test)]
mod tests {
    use crate::life_support_rating;

    #[test]
    fn day3_2_test() {
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

        assert_eq!(230, life_support_rating(test_data));
    }
}
