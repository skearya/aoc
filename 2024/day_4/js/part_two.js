import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf-8");

const grid = input
  .split("\n")
  .filter((x) => x !== "")
  .map((x) => [...x]);

const combinations = [
  ["M", "S", "M", "S"],
  ["S", "M", "S", "M"],
  ["S", "S", "M", "M"],
  ["M", "M", "S", "S"],
];

const check = ([x, y, c]) => (grid[x] !== undefined ? grid[x][y] === c : false);

let sum = 0;

for (let i = 0; i < grid.length; i++) {
  for (let j = 0; j < grid[i].length; j++) {
    if (grid[i][j] === "A") {
      for (const combination of combinations) {
        const upleft = [i + 1, j - 1, combination[0]];
        const upright = [i + 1, j + 1, combination[1]];
        const downleft = [i - 1, j - 1, combination[2]];
        const downright = [i - 1, j + 1, combination[3]];

        const valid = [upleft, upright, downleft, downright].every((x) =>
          check(x)
        );

        if (valid) {
          sum += 1;
          break;
        }
      }
    }
  }
}

console.log(sum);
