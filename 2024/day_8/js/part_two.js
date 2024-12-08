import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf-8");

const grid = input
  .trim()
  .split("\n")
  .map((line) => line.split(""));

const antinodes = [];

for (let i = 0; i < grid.length; i++) {
  for (let j = 0; j < grid[i].length; j++) {
    if (grid[i][j] !== ".") {
      const antennaAntinodes = grid
        .flatMap((row, i2) => row.map((item, j2) => [item, i2, j2]))
        .filter(([item, i2, j2]) => item === grid[i][j] && i !== i2 && j !== j2)
        .map(([_item, i2, j2]) => [i2 - i, j2 - j])
        .flatMap((diffs) => inlineAntinodes([i, j], diffs))
        .filter(
          ([i2, j2]) =>
            !antinodes.some((node) => node[0] === i2 && node[1] === j2)
        );

      antinodes.push(...antennaAntinodes);
    }
  }
}

function inlineAntinodes([row, col], [rowDiff, colDiff]) {
  const nodes = [];
  let i = 1;

  while (true) {
    const node = [row + rowDiff * i, col + colDiff * i];

    if (
      node[0] >= 0 &&
      node[1] >= 0 &&
      node[0] < grid.length &&
      node[1] < grid[0].length
    ) {
      nodes.push(node);
      i++;
    } else {
      break;
    }
  }

  return nodes;
}

console.log(antinodes.length);
