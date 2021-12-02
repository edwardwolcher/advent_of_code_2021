import * as fs from "fs";

const data_path =
  "/Users/ew/Library/Mobile Documents/com~apple~CloudDocs/Projects/advent_2021/data/day1.txt";

const test_input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

function count_increases(input_list: Array<number>): number {
  let increases = 0;
  for (let i = 0; i < input_list.length; i++) {
    if (i > 0 && input_list[i] > input_list[i - 1]) {
      increases++;
    }
  }
  return increases;
}

function read_input(path: string): Array<number> {
  return fs
    .readFileSync(path, "utf8")
    .split("\n")
    .map((line) => parseInt(line));
}

function test() {
  const result = count_increases(test_input);
  console.log(`Day 1_1 test: ${result} - ${result === 7}`);
}

function main() {
  const result = count_increases(read_input(data_path));
  console.log(`Day 1_1: ${result}`);
}

// test();
main();
