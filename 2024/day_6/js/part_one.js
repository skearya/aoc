import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf-8");

const grid = input
  .trim()
  .split("\n")
  .map((x) => [...x]);

let pos;

for (let i = 0; i < grid.length; i++) {
  for (let j = 0; j < grid[i].length; j++) {
    if (grid[i][j] === "^") {
      pos = [i, j];
    }
  }
}

const dirs = [
  [-1, 0], // Up
  [0, 1], // Right
  [1, 0], // Down
  [0, -1], // Left
];

let dir = [-1, 0];

const applyDir = ([row, col], dir) => [row + dir[0], col + dir[1]];

const turnRight = (dir) => {
  const index = dirs.findIndex(
    (other) => dir[0] === other[0] && dir[1] === other[1]
  );

  return dirs[(index + 1) % 4];
};

function traverse([row, col], dir) {
  const positions = [];

  while (true) {
    if (row >= grid.length || row < 0) break;
    if (col >= grid[0].length || col < 0) break;

    if (grid[row][col] === "#") {
      const oppositeDir = [dir[0] * -1, dir[1] * -1];

      [row, col] = applyDir([row, col], oppositeDir);
      dir = turnRight(dir);

      continue;
    }

    if (!positions.some((other) => other[0] === row && other[1] === col)) {
      positions.push([row, col]);
    }

    [row, col] = applyDir([row, col], dir);
  }

  return positions;
}

console.log(traverse(pos, dir).length);
