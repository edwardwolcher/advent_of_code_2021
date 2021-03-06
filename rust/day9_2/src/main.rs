// Note, this solution is copied from Timvisee's excellent Rust solutions to this year's advent at https://github.com/timvisee/advent-of-code-2021/blob/master/day08b/src/main.rs
// I will study the solution to improve my style.

const NEXT: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

pub fn main() {
    let mut input = include_bytes!(
        "/Users/ew/Library/Mobile Documents/com~apple~CloudDocs/Projects/advent_2021/data/day9.txt"
    )
    .split(|&b| b == b'\n')
    .map(|l| l.to_vec())
    .collect::<Vec<Vec<u8>>>();

    let mut basins = vec![];
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            (input[y][x] < b'9').then(|| basins.push(basin(&mut input, x, y)));
        }
    }

    basins.sort_unstable();
    println!("{}", basins.iter().rev().take(3).product::<usize>());
}

fn basin(map: &mut Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    map[y][x] = b'9';
    NEXT.iter()
        .map(|(xx, yy)| ((x as isize + xx) as usize, (y as isize + yy) as usize))
        .fold(1, |acc, (x, y)| {
            match map.get(y).and_then(|l| l.get(x)).map(|&n| n < b'9') {
                Some(true) => acc + basin(map, x, y),
                _ => acc,
            }
        })
}
