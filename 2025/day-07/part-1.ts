import "cheats";
import { OrderedSet } from "cheats";
import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf-8")
	.lines()
	.truthy()
	.map((line) => [...line]);

const beams = new OrderedSet<[number, number]>(
	([row, col]) => `${row}, ${col}`
);

beams.push([0, input[0].findIndex((c) => c === "S")]);

let split = 1;

while (beams.length !== 0) {
	const [row, col] = beams.shift()!;

	const [newRow, newCol] = [row + 1, col];
	if (newRow >= input.length) continue;

	if (input[newRow][newCol] === "^") {
		beams.push([newRow, newCol - 1]);
		beams.push([newRow, newCol + 1]);

		split = split * 2;
	} else if (input[newRow][newCol] === ".") {
		beams.push([newRow, newCol]);
	}
}

console.log(split);
