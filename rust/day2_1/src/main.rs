use std::fs;

pub const DATA_PATH: &str =
    "/Users/ew/Library/Mobile Documents/com~apple~CloudDocs/Projects/advent_2021/data/day2.txt";

enum Direction {
    Forward,
    Down,
    Up,
}

struct Position {
    depth: i32,
    horizontal: i32,
}

impl Position {
    fn execute(&mut self, command: Command) {
        match command.direction {
            Direction::Forward => self.horizontal += command.distance,
            Direction::Down => self.depth += command.distance,
            Direction::Up => self.depth -= command.distance,
        }
    }
}
struct Command {
    direction: Direction,
    distance: i32,
}

impl Command {
    fn new(command_string: &str) -> Self {
        let mut instruction_iter = command_string.split_whitespace();
        let command_string = instruction_iter.next().unwrap();
        let distance = instruction_iter.next().unwrap().parse::<i32>().unwrap();
        let direction = match command_string {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => panic!("Error Reading Data"),
        };
        Command {
            direction,
            distance,
        }
    }
}

fn parse_commands(commands_string: String) -> Vec<Command> {
    let mut commands = Vec::new();
    for command_string in commands_string.lines() {
        commands.push(Command::new(command_string));
    }
    commands
}

fn main() {
    let input_string = fs::read_to_string(DATA_PATH).expect("error reading data");
    let commands = parse_commands(input_string);
    let mut position = Position {
        depth: 0,
        horizontal: 0,
    };
    for command in commands {
        position.execute(command);
    }
    println!(
        "Horizontal Position: {}, Depth: {}, product: {}",
        position.horizontal,
        position.depth,
        position.horizontal * position.depth
    );
}

#[cfg(test)]
mod tests {
    use crate::parse_commands;
    use crate::Position;

    #[test]
    fn day2_1_test() {
        let test_data = String::from(
            "forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2",
        );
        let commands = parse_commands(test_data);
        let mut position = Position {
            depth: 0,
            horizontal: 0,
        };
        for command in commands {
            position.execute(command)
        }
        assert_eq!(position.horizontal, 15);
        assert_eq!(position.depth, 10);
    }
}
