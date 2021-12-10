use std::cmp::*;
use std::fs;

pub const DATA_PATH: &str =
    "/Users/ew/Library/Mobile Documents/com~apple~CloudDocs/Projects/advent_2021/data/day5.txt";

fn main() {
    let input_string = fs::read_to_string(DATA_PATH).expect("error reading data");
    let lines = parse_input(input_string);

    let mut ocean = Ocean::new(1000, 1000);
    for line in lines {
        ocean.draw_line(line);
    }

    println!("Solution: {}", ocean.two_overlaps());
}
#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
    intersects: usize,
}

#[derive(Debug)]
struct Line {
    point_a: (usize, usize),
    point_b: (usize, usize),
}

fn parse_input(input: String) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(" ").collect();
            let point_a: Vec<&str> = split[0].split(",").collect();
            let point_a: (usize, usize) = (
                point_a[0].parse::<usize>().unwrap(),
                point_a[1].parse::<usize>().unwrap(),
            );
            let point_b: Vec<&str> = split[2].split(",").collect();
            let point_b: (usize, usize) = (
                point_b[0].parse::<usize>().unwrap(),
                point_b[1].parse::<usize>().unwrap(),
            );
            Line { point_a, point_b }
        })
        .collect()
}

impl Line {
    fn not_diag(&self) -> bool {
        self.point_a.1 == self.point_b.1 || self.point_a.0 == self.point_b.0
    }
    fn points(&self) -> Vec<(usize, usize)> {
        let mut points = Vec::new();
        if self.not_diag() {
            if self.point_a.0 != self.point_b.0 {
                let (smaller_x, bigger_x) = (
                    min(self.point_a.0, self.point_b.0),
                    max(self.point_a.0, self.point_b.0) + 1,
                );
                for x in smaller_x..bigger_x {
                    points.push((x, self.point_a.1));
                }
            } else {
                let (smaller_y, bigger_y) = (
                    min(self.point_a.1, self.point_b.1),
                    max(self.point_a.1, self.point_b.1) + 1,
                );
                for y in smaller_y..bigger_y {
                    points.push((self.point_a.0, y));
                }
            }
        } else {
            let (smaller_x, bigger_x) = if self.point_a.0 > self.point_b.0 {
                (self.point_b, self.point_a)
            } else {
                (self.point_a, self.point_b)
            };
            for x in smaller_x.0..bigger_x.0 + 1 {
                let direction: i32 = if smaller_x.1 < bigger_x.1 { 1 } else { -1 };
                let y = (smaller_x.1 as i32 + (x - smaller_x.0) as i32 * direction) as usize;
                points.push((x, y));
            }
        }
        points
    }
}

#[derive(Debug)]
struct Ocean {
    rows: Vec<Vec<Point>>,
}

impl Ocean {
    fn new(max_x: usize, max_y: usize) -> Self {
        let mut rows: Vec<Vec<Point>> = Vec::new();
        for y in 0..max_y {
            let mut row: Vec<Point> = Vec::new();
            for x in 0..max_x {
                row.push(Point {
                    x: x,
                    y: y,
                    intersects: 0,
                });
            }
            rows.push(row);
        }
        Ocean { rows: rows }
    }
    fn draw_line(&mut self, line: Line) {
        let points = line.points();
        for point in points {
            self.rows[point.1][point.0].intersects += 1;
        }
    }
    fn two_overlaps(&mut self) -> usize {
        self.rows.iter().fold(0, |acc, row| {
            acc + row.iter().filter(|point| point.intersects > 1).count()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_input;
    use crate::Ocean;

    #[test]
    fn day5_2_test() {
        let test_input = String::from(
            "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
        );

        let lines = parse_input(test_input);

        let mut ocean = Ocean::new(10, 10);
        for line in lines {
            ocean.draw_line(line);
        }

        assert_eq!(12, ocean.two_overlaps())
    }
}
