import "cheats";
import { readFileSync } from "node:fs";

const [unparsedFreshRanges] = readFileSync("input.txt", "utf-8").split("\n\n");

const freshRanges = unparsedFreshRanges
	.lines()
	.map((line) => line.split("-").ints());

freshRanges.sort(([a], [b]) => a - b);

const mergedRanges: [number, number][] = [];

let i = 0;

while (i < freshRanges.length) {
	let [l, r] = freshRanges[i];

	while (i + 1 < freshRanges.length && r >= freshRanges[i + 1][0]) {
		r = Math.max(r, freshRanges[i + 1][1]);
		i++;
	}

	mergedRanges.push([l, r]);
	i++;
}

const fresh = mergedRanges.reduce((sum, [l, r]) => sum + r - l + 1, 0);

console.log(fresh);
