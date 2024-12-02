import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf-8");

let nums = input
  .split("\n")
  .filter((x) => x !== "")
  .map((x) => x.split(" ").map((x) => parseInt(x)));

let correct = 0;

for (let i = 0; i < nums.length; i++) {
  if (isValid(nums[i])) {
    correct += 1;
  } else {
    for (let i2 = 0; i2 < nums[i].length; i2++) {
      let clone = [...nums[i]];
      clone.splice(i2, 1);

      if (isValid(clone)) {
        correct += 1;
        break;
      }
    }
  }
}

function isValid(nums) {
  let diffs = [];

  for (let j = 0; j < nums.length; j++) {
    let cur = nums[j];
    let next = nums[j + 1];

    if (next == undefined) {
      break;
    }

    diffs.push(cur - next);
  }

  let bad = diffs.some((x) => Math.abs(x) < 1 || Math.abs(x) > 3);
  let neg = diffs.every((x) => x < 0);
  let pos = diffs.every((x) => x >= 0);

  if (bad) {
    return false;
  }

  return (neg && !pos) || (pos && !neg);
}

console.log(correct);
