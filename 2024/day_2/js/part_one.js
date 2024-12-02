import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf-8");

let nums = input
  .split("\n")
  .filter((x) => x !== "")
  .map((x) => x.split(" ").map((x) => parseInt(x)));

let correct = 0;

for (let i = 0; i < nums.length; i++) {
  let diffs = [];

  for (let j = 0; j < nums[i].length; j++) {
    let cur = nums[i][j];
    let next = nums[i][j + 1];

    if (next == undefined) {
      break;
    }

    diffs.push(cur - next);
  }

  let bad = diffs.some((x) => Math.abs(x) < 1 || Math.abs(x) > 3);
  let neg = diffs.every((x) => x < 0);
  let pos = diffs.every((x) => x >= 0);

  if (bad) {
    continue;
  }

  if ((neg && !pos) || (pos && !neg)) {
    correct += 1;
  }
}

console.log(correct);
