import "cheats";
import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf-8").lines().truthy();

const symbols = input.pop()!;
const numbers = input;

const rowLength = numbers[0].length;

let total = 0;
let i = 0;

while (i < rowLength) {
	let op = symbols.charAt(i);
	let acc = op === "*" ? 1 : 0;

	while (i < rowLength && numbers.some((line) => line.charAt(i) !== " ")) {
		let builder = "";

		for (const line of numbers) {
			const char = line.charAt(i);
			if (char === " ") continue;

			builder += char;
		}

		const number = parseInt(builder);

		acc = op === "*" ? acc * number : acc + number;
		i++;
	}

	total += acc;
	i++;
}

console.log(total);
