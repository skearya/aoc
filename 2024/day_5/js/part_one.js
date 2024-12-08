import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf-8");

const map = {};

let [rules, updates] = input
  .split("\n\n")
  .map((x) => x.split("\n").filter((x) => x !== ""));

rules = rules.map((x) => x.split("|").map(Number));
updates = updates.map((x) => x.split(",").map(Number));

for (const [before, after] of rules) {
  map[before] ??= { before: [], after: [] };
  map[before].after.push(after);

  map[after] ??= { before: [], after: [] };
  map[after].before.push(before);
}

const validUpdate = (update) => {
  for (let i = 0; i < update.length; i++) {
    const before = update.slice(0, i);
    const after = update.slice(i + 1);

    const passed =
      before.every((x) => map[update[i]].before.includes(x)) &&
      after.every((x) => map[update[i]].after.includes(x));

    if (!passed) {
      return false;
    }
  }

  return true;
};

console.log(
  updates
    .filter((update) => validUpdate(update))
    .map((update) => update[Math.floor(update.length / 2)])
    .reduce((acc, x) => acc + x)
);
