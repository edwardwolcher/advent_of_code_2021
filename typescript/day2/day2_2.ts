import * as fs from "fs";

const DATA_PATH =
  "/Users/ew/Library/Mobile Documents/com~apple~CloudDocs/Projects/advent_2021/data/day2.txt";

class Position {
  horizontal: number;
  depth: number;
  aim: number;

  constructor() {
    this.horizontal = 0;
    this.depth = 0;
    this.aim = 0;
  }
  execute(command: Command) {
    switch (command.direction) {
      case "down":
        this.aim += command.distance;
        break;
      case "up":
        this.aim -= command.distance;
        break;
      case "forward":
        this.horizontal += command.distance;
        this.depth += this.aim * command.distance;
        break;
    }
  }
  report() {
    console.log(
      `Horizontal: ${this.horizontal}, Depth: ${this.depth}, Product: ${
        this.horizontal * this.depth
      }`
    );
  }
}
class Command {
  direction: string;
  distance: number;

  constructor(command_string: string) {
    const command_strings = command_string.split(" ");
    this.direction = command_strings[0];
    this.distance = parseInt(command_strings[1]);
  }
}

function parse_commands(commands_string: String): Array<Command> {
  return commands_string
    .split("\n")
    .map((command_string) => new Command(command_string));
}

function main() {
  const command_data = fs.readFileSync(DATA_PATH, "utf-8");
  const commands = parse_commands(command_data);
  const position = new Position();
  commands.forEach((command) => {
    position.execute(command);
  });
  position.report();
}

function test() {
  const test_data = `forward 5
down 5
forward 8
up 3
down 8
forward 2`;

  const position = new Position();
  const commands = parse_commands(test_data);
  commands.forEach((command) => {
    position.execute(command);
  });
  position.report();
}

console.log("\nTEST:");
test();
console.log("\n---\n");
console.log("MAIN:");
main();
