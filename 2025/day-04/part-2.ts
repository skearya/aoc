import "cheats";
import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf-8");

const grid = input
	.lines()
	.truthy()
	.map((line) => [...line]);

const directions = [-1, 0, 1]
	.flatMap((dy) => [-1, 0, 1].map((dx) => [dy, dx] as const))
	.filter(([dy, dx]) => !(dy === 0 && dx === 0));

function cleanup(): number {
	let cleaned = 0;

	for (let row = 0; row < grid.length; row++) {
		for (let col = 0; col < grid[row].length; col++) {
			if (grid[row][col] !== "@") continue;

			let adjacent = 0;

			for (const [dy, dx] of directions) {
				const newRow = row + dy;
				if (newRow < 0 || newRow >= grid.length) continue;

				const newCol = col + dx;
				if (newCol < 0 || newCol >= grid[row].length) continue;

				if (grid[newRow][newCol] === "@") adjacent++;
				if (adjacent === 4) break;
			}

			if (adjacent < 4) {
				grid[row][col] = ".";
				cleaned++;
			}
		}
	}

	return cleaned;
}

let removed = 0;

while (true) {
	const cleaned = cleanup();
	if (cleaned === 0) break;

	removed += cleaned;
}

console.log(removed);
