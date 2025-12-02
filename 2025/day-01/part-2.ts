import "cheats";
import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf-8");

const rotations = input
	.lines()
	.truthy()
	.map((l) => [l.charAt(0), parseInt(l.substring(1))] as const);

let dial = 50;
let count = 0;

for (const [dir, rotation] of rotations) {
	let prev = dial;
	dial = dir === "L" ? dial - rotation : dial + rotation;

	while (true) {
		if (dial < 0) {
			dial += 100;
			if (prev !== 0) count++;
		} else if (dial > 99) {
			dial -= 100;
			if (dial !== 0) count++;
		} else if (dial === 0) {
			count++;
			break;
		} else {
			break;
		}

		prev = dial;
	}
}

console.log(dial, count);
