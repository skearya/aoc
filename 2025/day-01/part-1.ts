import "../utils.ts";
import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf-8");

const rotations = input
	.lines()
	.truthy()
	.map((l) => [l.charAt(0), parseInt(l.substring(1))] as const);

let dial = 50;
let count = 0;

for (const [dir, rotation] of rotations) {
	dial = dir === "L" ? dial - rotation : dial + rotation;

	while (true) {
		if (dial < 0) {
			dial += 100;
		} else if (dial > 99) {
			dial -= 100;
		} else {
			break;
		}
	}

	if (dial === 0) count++;
}

console.log(dial, count);
