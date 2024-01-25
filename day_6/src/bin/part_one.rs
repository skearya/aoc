fn main() {
    let lines: Vec<&str> = include_str!("../input.txt").lines().collect();

    let (times, distances) = (get_numbers(lines[0]), get_numbers(lines[1]));
    let races: Vec<(u16, u16)> = times.into_iter().zip(distances).collect();

    let margin_of_error: u32 = races
        .iter()
        .map(|(time, distance)| {
            let mut winning_ways = 0;

            for ms_pressed in 0..*time {
                let remaining_time = time - ms_pressed;
                let distance_traveled = ms_pressed * remaining_time;

                if distance_traveled > *distance {
                    winning_ways += 1;
                }
            }

            winning_ways
        })
        .product();

    println!("{}", margin_of_error)
}

fn get_numbers(line: &str) -> Vec<u16> {
    line.split_once(":")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}
