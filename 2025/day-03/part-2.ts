import "cheats";
import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf-8");

const banks = input
	.lines()
	.truthy()
	.map((line) => [...line].ints());

let total = 0;

for (const bank of banks) {
	let joltage = 0;
	let lastIndex = -1;

	for (let battery = 11; battery >= 0; battery--) {
		let max = 0;
		let maxIndex = 0;

		for (let i = lastIndex + 1; i < bank.length - battery; i++) {
			if (bank[i] > max) {
				max = bank[i];
				maxIndex = i;
			}
		}

		joltage += max * Math.pow(10, battery);
		lastIndex = maxIndex;
	}

	total += joltage;
}

console.log(total);
