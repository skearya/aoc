import "cheats";
import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf-8");

const ranges = input.split(",").map((range) => range.split("-").ints());

let sum = 0;

for (const [start, end] of ranges) {
	for (let number = start; number <= end; number++) {
		const digits = Math.floor(Math.log10(number)) + 1;
		if (digits % 2 === 1) continue;

		const halfDigits = digits / 2;
		const leftHalf = Math.floor(number / Math.pow(10, halfDigits));
		const rightHalf = number - leftHalf * Math.pow(10, halfDigits);

		if (leftHalf === rightHalf) sum += number;
	}
}

console.log(sum);
