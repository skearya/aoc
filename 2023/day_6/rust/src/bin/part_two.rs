fn main() {
    let lines: Vec<&str> = include_str!("../../../input.txt").lines().collect();

    let (time, distance) = (get_number(lines[0]), get_number(lines[1]));

    let mut winning_ways = 0;

    for ms_pressed in 0..time {
        let remaining_time = time - ms_pressed;
        let distance_traveled = ms_pressed * remaining_time;

        if distance_traveled > distance {
            winning_ways += 1;
        }
    }

    println!("{}", winning_ways)
}

fn get_number(line: &str) -> u64 {
    line.split_once(":")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .flat_map(|num| num.chars())
        .collect::<String>()
        .parse()
        .unwrap()
}
