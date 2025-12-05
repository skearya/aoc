import "cheats";
import { readFileSync } from "node:fs";

const [unparsedFreshRanges, unparsedIngredients] = readFileSync(
	"input.txt",
	"utf-8"
).split("\n\n");

const freshRanges = unparsedFreshRanges
	.lines()
	.map((line) => line.split("-").ints());

const ingredients = unparsedIngredients.lines().ints();

const freshCount = ingredients.filter((id) =>
	freshRanges.some(([l, r]) => l <= id && id <= r)
).length;

console.log(freshCount);
