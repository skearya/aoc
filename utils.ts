declare global {
	interface String {
		lines(limit?: number): string[];
		spaces(limit?: number): string[];
	}

	interface Array<T> {
		truthy(): T[];
		ints(radix?: number): number[];
		floats(): number[];
		sum(): number;
		product(): number;
		unique(): T[];
		descending(): T[];
		ascending(): T[];
		descendingBy(f: (arg: T) => number): T[];
		ascendingBy(f: (arg: T) => number): T[];
		interleave<U>(other: U[]): (T | U)[];
		permutations(): T[][];
	}
}

export function range(start: number, stop?: number, step = 1) {
	if (stop === undefined) {
		stop = start;
		start = 0;
	}

	return Array.from(
		{ length: Math.ceil((stop - start) / step) },
		(_, i) => start + i * step
	);
}

function define<T>(object: T, fn: (this: T, ...args: any[]) => void) {
	Object.defineProperty(object, fn.name, {
		writable: false,
		configurable: false,
		enumerable: false,
		value: fn,
	});
}

define(String.prototype, function lines(limit) {
	return this.split("\n", limit);
});

define(String.prototype, function spaces(limit) {
	return this.split(/\s+/, limit);
});

define(Array.prototype, function truthy() {
	return this.filter((x) => x);
});

define(Array.prototype, function ints(radix) {
	return this.map((s) => parseInt(s, radix));
});

define(Array.prototype, function floats() {
	return this.map((s) => parseFloat(s));
});

define(Array.prototype, function sum() {
	return this.reduce((acc, n) => acc + n);
});

define(Array.prototype, function product() {
	return this.reduce((acc, n) => acc * n);
});

define(Array.prototype, function unique() {
	return [...new Set(this)];
});

define(Array.prototype, function descending() {
	return this.toSorted((a, b) => b - a);
});

define(Array.prototype, function ascending() {
	return this.toSorted((a, b) => a - b);
});

define(Array.prototype, function descendingBy(f) {
	return this.toSorted((a, b) => f(b) - f(a));
});

define(Array.prototype, function ascendingBy(f) {
	return this.toSorted((a, b) => f(a) - f(b));
});

define(Array.prototype, function interleave(other) {
	const result = [];

	let i = 0;
	let j = 0;

	while (i < this.length || j < other.length) {
		if (i < this.length) {
			result.push(this[i]);
			i++;
		}
		if (j < other.length) {
			result.push(other[j]);
			j++;
		}
	}

	return result;
});

define(Array.prototype, function permutations() {
	// Source - https://stackoverflow.com/a/37580979
	// Posted by le_m, modified by community. See post 'Timeline' for change history
	// Retrieved 2025-11-28, License - CC BY-SA 4.0
	var length = this.length,
		result = [this.slice()],
		c = new Array(length).fill(0),
		i = 1,
		k,
		p;

	while (i < length) {
		if (c[i] < i) {
			k = i % 2 && c[i];
			p = this[i];
			this[i] = this[k];
			this[k] = p;
			++c[i];
			i = 1;
			result.push(this.slice());
		} else {
			c[i] = 0;
			++i;
		}
	}

	return result;
});

export class OrderedSet<T> {
	set: Set<string>;
	array: T[];
	stringify: (item: T) => string;

	constructor(stringify: (item: T) => string) {
		this.set = new Set();
		this.array = [];
		this.stringify = stringify;
	}

	get content() {
		return this.array;
	}

	get length() {
		return this.array.length;
	}

	push(item: T) {
		const key = this.stringify(item);

		if (!this.set.has(key)) {
			this.array.push(item);
			this.set.add(key);
		}
	}

	pop(): T | undefined {
		const item = this.array.pop();
		if (item) this.set.delete(this.stringify(item));

		return item;
	}

	shift(): T | undefined {
		const item = this.array.shift();
		if (item) this.set.delete(this.stringify(item));

		return item;
	}
}
