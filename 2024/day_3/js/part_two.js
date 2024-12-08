import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf-8");

let sum = 0;
let enabled = true;

let matches = input.match(/mul\(\d+,\d+\)|do\(\)|don't\(\)/g);

for (const match of matches) {
  if (match.startsWith("do()")) {
    enabled = true;
  } else if (match.startsWith("don't()")) {
    enabled = false;
  } else if (enabled) {
    const nums = match
      .substring(4)
      .substring(0, match.length - 5)
      .split(",")
      .map((x) => parseInt(x, 10));

    sum += nums[0] * nums[1];
  }
}

console.log(sum);
