import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf-8");

let nums = input.split("\n").map((x) =>
  x
    .split(" ")
    .filter((x) => x !== "")
    .map((x) => parseInt(x, 10))
);

let left = nums.map((x) => x[0]).sort((a, b) => a - b);
let right = nums.map((x) => x[1]).sort((a, b) => a - b);

let sum = 0;

for (let i = 0; i < left.length; i++) {
  sum += Math.abs(left[i] - right[i]);
}

console.log(sum);
