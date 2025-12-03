import "cheats";
import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf-8");

const banks = input.lines().map((line) => [...line].ints());

let total = 0;

for (const bank of banks) {
	let max = 0;
	let maxIndex = 0;

	for (let i = 0; i < bank.length - 1; i++) {
		if (bank[i] > max) {
			max = bank[i];
			maxIndex = i;
		}
	}

	let secondMax = 0;

	for (let i = maxIndex + 1; i < bank.length; i++) {
		secondMax = Math.max(secondMax, bank[i]);
	}

	total += max * 10 + secondMax;
}

console.log(total);
