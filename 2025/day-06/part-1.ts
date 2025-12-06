import "cheats";
import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf-8")
	.lines()
	.filter((line) => line.length !== 0)
	.map((line) => line.spaces().truthy());

const symbols = input.pop()!;
const numbers = input.map((nums) => nums.ints());

const total = symbols
	.map((op, i) =>
		numbers.reduce(
			(acc, line) => (op === "*" ? acc * line[i] : acc + line[i]),
			op === "*" ? 1 : 0
		)
	)
	.sum();

console.log(total);
