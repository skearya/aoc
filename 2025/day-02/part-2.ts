import "cheats";
import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf-8");

const ranges = input.split(",").map((range) => range.split("-").ints());

let sum = 0;

for (const [start, end] of ranges) {
	for (let number = start; number <= end; number++) {
		const digits = Math.floor(Math.log10(number)) + 1;
		if (digits < 2) continue;

		for (let splits = 2; splits <= digits; splits++) {
			if (splitNumber(number, splits, digits)) {
				sum += number;
				break;
			}
		}
	}
}

console.log(sum);

function splitNumber(number: number, splits: number, digits: number): boolean {
	const splitSize = digits / splits;
	if (splitSize !== Math.floor(splitSize)) return false;

	const shifted = number / Math.pow(10, 0);
	const filtered = shifted % Math.pow(10, splitSize);
	const floored = Math.floor(filtered);

	const firstSplit = floored;

	for (let i = splitSize; i < digits; i += splitSize) {
		const shifted = number / Math.pow(10, i);
		const filtered = shifted % Math.pow(10, splitSize);
		const floored = Math.floor(filtered);

		if (floored !== firstSplit) {
			return false;
		}
	}

	return true;
}
