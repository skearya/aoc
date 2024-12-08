import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf-8");

const grid = input
  .split("\n")
  .filter((x) => x !== "")
  .map((x) => [...x]);

const rest = ["M", "A", "S"];

const check = (indexes) => indexes.every(([x, y], i) => grid[x] !== undefined ? grid[x][y] === rest[i] : false);

let sum = 0;

for (let i = 0; i < grid.length; i++) {
  for (let j = 0; j < grid[i].length; j++) {
    if (grid[i][j] === "X") {
      const up = [[i + 1, j], [i + 2, j], [i + 3, j]];
      const down = [[i - 1, j], [i - 2, j], [i - 3, j]];
      const left = [[i, j + 1], [i, j + 2], [i, j + 3]];
      const right = [[i, j - 1], [i, j - 2], [i, j - 3]];

      const upleft = [[i + 1, j - 1], [i + 2, j - 2], [i + 3, j - 3]];
      const upright = [[i + 1, j + 1], [i + 2, j + 2], [i + 3, j + 3]];
      const downleft = [[i - 1, j - 1], [i - 2, j - 2], [i - 3, j - 3]];
      const downright = [[i - 1, j + 1], [i - 2, j + 2], [i - 3, j + 3]];

      sum += [up, down, left, right, upleft, upright, downleft, downright].filter(check).length;
    }
  }
}

console.log(sum);
