use std::collections::HashMap;

fn main() {
    let mut lines = include_str!("../input.txt").lines();

    let seeds: Vec<u64> = lines
        .nth(0)
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .flat_map(|num| num.parse())
        .collect();

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

    let locations: Vec<u64> = seeds
        .iter()
        .map(|seed| {
            let (where_to, ranges) = &maps["seed"];

            let mut current = convert_source(*seed, ranges);
            let mut to = where_to;

            while let Some((where_to, ranges)) = maps.get(to) {
                current = convert_source(current, ranges);
                to = where_to
            }

            current
        })
        .collect();

    println!("lowest num: {}", locations.iter().min().unwrap())
}

fn convert_source(num: u64, ranges: &Vec<(u64, u64, u64)>) -> u64 {
    for (destination, source, range_len) in ranges {
        let range = *source..source + range_len;

        if range.contains(&num) {
            return num - source + destination;
        }
    }

    num
}
