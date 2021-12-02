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

function map_to_runs(input_list: Array<number>): Array<number> {
  const runs: Array<number> = [];
  for (let i = 0; i < input_list.length - 2; i++) {
    const run = input_list[i] + input_list[i + 1] + input_list[i + 2];
    runs.push(run);
  }
  return runs;
}

function test() {
  const result = count_increases(map_to_runs(test_input));
  console.log(`Day 1_2 test: ${result} - ${result === 5}`);
}

function main() {
  const result = count_increases(map_to_runs(read_input(data_path)));
  console.log(`Day 1_2: ${result}`);
}

// test();
main();
