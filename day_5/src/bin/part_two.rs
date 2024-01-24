fn main() {
    let mut lines = include_str!("../input.txt").lines();

    let seeds: Vec<u64> = lines
        .nth(0)
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .flat_map(|num| num.parse::<u64>())
        .collect();

    let seed_ranges: Vec<(u64, u64)> = seeds
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();

    let _ = lines.next();

    let mut maps: Vec<Vec<(u64, u64, u64)>> = Vec::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            let _ = lines.next();
            continue;
        }

        let mut instructions = Vec::new();

        while let Some(instruction) = lines
            .next()
            .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        {
            if instruction.len() == 0 {
                break;
            }

            let parsed: Vec<u64> = instruction.iter().flat_map(|num| num.parse()).collect();

            instructions.push((parsed[0], parsed[1], parsed[2]))
        }

        maps.push(instructions);
    }

    let mut lowest_location: u64 = u64::MAX;

    for (start, len) in seed_ranges {
        for num in start..start + len {
            let result = to_location(num, 0, &maps);

            if result < lowest_location {
                lowest_location = result
            }
        }
    }

    println!("lowest num: {}", lowest_location)
}

fn to_location(num: u64, i: usize, maps: &Vec<Vec<(u64, u64, u64)>>) -> u64 {
    if i == 7 {
        return num;
    }

    return to_location(convert_range(num, &maps[i]), i + 1, maps);
}

fn convert_range(num: u64, ranges: &Vec<(u64, u64, u64)>) -> u64 {
    for (destination, source, range_len) in ranges {
        if (*source..source + range_len).contains(&num) {
            return num - source + destination;
        }
    }

    num
}
