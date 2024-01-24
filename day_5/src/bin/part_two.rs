use std::collections::HashMap;

fn main() {
    let mut lines = include_str!("../input.txt").lines();

    let mut seeds = lines
        .nth(0)
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .flat_map(|num| num.parse::<u64>());

    let mut seed_ranges: Vec<(u64, u64)> = Vec::new();

    while let Some(seed) = seeds.next() {
        seed_ranges.push((seed, seeds.next().unwrap()));
    }

    let _ = lines.next();

    let mut maps: HashMap<&str, (&str, Vec<(u64, u64, u64)>)> = HashMap::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            let _ = lines.next();
            continue;
        }

        let map_name: Option<(&str, &str)> = line.split_once(" map:").map(|line| {
            let elements: Vec<&str> = line.0.split("-").collect();
            (elements[0], elements[2])
        });

        if let Some((from, to)) = map_name {
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

            maps.insert(from, (to, instructions));
        }
    }

    let mut lowest_location: u64 = u64::MAX;

    for (start, len) in seed_ranges {
        for num in start..start + len {
            let result = to_location(num, "seed", &maps);
            
            if result < lowest_location {
                lowest_location = result
            }
        }
    }

    println!("lowest num: {}", lowest_location)
}

fn to_location(num: u64, to: &str, maps: &HashMap<&str, (&str, Vec<(u64, u64, u64)>)>) -> u64 {
    if let Some((where_to, ranges)) = maps.get(to) {
        return to_location(convert_range(num, ranges), where_to, maps);
    }

    num
}

fn convert_range(num: u64, ranges: &Vec<(u64, u64, u64)>) -> u64 {
    for (destination, source, range_len) in ranges {
        let range = *source..source + range_len;

        if range.contains(&num) {
            return num - source + destination;
        }
    }

    num
}
