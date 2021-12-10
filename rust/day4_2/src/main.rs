use std::fs;

pub const DATA_PATH: &str =
    "/Users/ew/Library/Mobile Documents/com~apple~CloudDocs/Projects/advent_2021/data/day4.txt";
fn main() {
    let input_string = fs::read_to_string(DATA_PATH).expect("error reading data");
    let (draws, bingo_boards) = process_input(input_string);
    let solution = get_solution(draws, bingo_boards);
    println!("Solution: {}", solution)
}

#[derive(Debug, Clone)]
struct BingoBoard {
    board: Vec<Vec<u32>>,
    hits: Vec<Vec<bool>>,
    won: bool,
}

impl BingoBoard {
    fn new(board_string: String) -> Self {
        let mut board: Vec<Vec<u32>> = Vec::new();

        let lines: Vec<&str> = board_string.lines().collect();
        for line in lines {
            let lines_split = line.split_whitespace();
            let row = lines_split
                .map(|num_str| num_str.parse().unwrap())
                .collect();
            board.push(row);
        }
        let mut hits: Vec<Vec<bool>> = Vec::new();
        for _ in 0..board.len() {
            hits.push(vec![false; board[0].len()])
        }

        BingoBoard {
            board: board,
            hits: hits,
            won: false,
        }
    }
    fn check_win(&self) -> bool {
        for i in 0..self.hits.len() {
            if self.hits[i].iter().all(|&hit| hit == true) {
                return true;
            }
        }
        for i in 0..self.hits[0].len() {
            if self.hits.iter().map(|row| &row[i]).all(|&hit| hit == true) {
                return true;
            }
        }
        false
    }
    fn update(&mut self, num: u32) -> Option<u32> {
        if self.won {
            return None;
        }
        for y in 0..self.board.len() {
            for x in 0..self.board[0].len() {
                if self.board[y][x] == num {
                    self.hits[y][x] = true;
                }
            }
        }
        if self.check_win() {
            self.won = true;
            Some(self.get_score())
        } else {
            None
        }
    }
    fn get_score(&self) -> u32 {
        let mut solution: u32 = 0;
        for y in 0..self.board.len() {
            for x in 0..self.board[0].len() {
                if !self.hits[y][x] {
                    solution += self.board[y][x];
                }
            }
        }
        solution
    }
}

fn process_input(input: String) -> (Vec<u32>, Vec<BingoBoard>) {
    let input_lines: Vec<&str> = input.split("\n\n").collect();

    let draws: Vec<u32> = input_lines[0]
        .split(',')
        .map(|draw| draw.parse::<u32>().unwrap())
        .collect();

    let mut bingo_boards: Vec<BingoBoard> = Vec::new();

    for i in 1..input_lines.len() {
        bingo_boards.push(BingoBoard::new(String::from(input_lines[i])));
    }

    (draws, bingo_boards)
}

fn get_solution(draws: Vec<u32>, mut bingo_boards: Vec<BingoBoard>) -> u32 {
    let mut score = 0;
    for draw in draws {
        for board in bingo_boards.iter_mut() {
            match board.update(draw) {
                None => (),
                Some(s) => score = s * draw,
            }
        }
    }

    score
}
#[cfg(test)]
mod tests {
    use crate::get_solution;
    use crate::process_input;

    #[test]
    fn day4_2_test() {
        let test_input = String::from(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7",
        );

        let (draws, bingo_boards) = process_input(test_input);
        let x = get_solution(draws, bingo_boards);
        assert_eq!(x, 1924)
    }
}
