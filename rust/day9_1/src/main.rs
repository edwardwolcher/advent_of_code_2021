#![allow(dead_code)]

const INPUT: &'static str = include_str!(
    "/Users/ew/Library/Mobile Documents/com~apple~CloudDocs/Projects/advent_2021/data/day9.txt"
);

fn main() {
    let height_map = HeightMap::new(INPUT);
    println!("Day 9_1 Solution: {}", height_map.low_points_sums());
}
#[derive(Debug)]
struct HeightMap {
    rows: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl HeightMap {
    fn new(input_str: &str) -> Self {
        let mut rows: Vec<Vec<u32>> = Vec::new();

        input_str.lines().for_each(|line| {
            let mut row = Vec::new();
            line.chars().for_each(|c| row.push(c.to_digit(10).unwrap()));
            rows.push(row);
        });

        HeightMap {
            height: rows.len(),
            width: rows[0].len(),
            rows,
        }
    }

    fn is_low(&self, x: usize, y: usize) -> bool {
        let value = self.rows[y][x];
        let left = if x > 0 { self.rows[y][x - 1] } else { 10 };
        let right = if x < self.width - 1 {
            self.rows[y][x + 1]
        } else {
            10
        };
        let up = if y > 0 { self.rows[y - 1][x] } else { 10 };
        let down = if y < self.height - 1 {
            self.rows[y + 1][x]
        } else {
            10
        };

        value < left && value < right && value < up && value < down
    }

    fn low_points_sums(&self) -> u32 {
        let mut sum = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_low(x, y) {
                    sum += 1 + self.rows[y][x];
                }
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::HeightMap;

    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn day9_1_test() {
        let height_map = HeightMap::new(INPUT);
        assert_eq!(15, height_map.low_points_sums());
    }
}
