import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf-8");

let result = input
  .match(/mul\(\d+,\d+\)/g)
  .map((x) => x.substring(4))
  .map((x) => x.substring(0, x.length - 1))
  .map((x) => x.split(",").map((x) => parseInt(x, 10)))
  .map((x) => x[0] * x[1])
  .reduce((acc, x) => acc + x);

console.log(result);
