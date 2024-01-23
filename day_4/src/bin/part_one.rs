fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(parse_line)
            .sum::<u32>()
    );
}

fn parse_line(line: &str) -> u32 {
    let (winning_numbers, mut numbers) = {
        let card = line.split_once(": ").unwrap().1;

        let mut table = card.split(" | ").map(|list| {
            list.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u32>>()
        });

        (table.next().unwrap(), table.next().unwrap())
    };

    numbers.retain(|n| winning_numbers.contains(n));

    if numbers.len() == 0 {
        return 0;
    }

    (0..numbers.len() - 1).fold(1, |acc, _| acc * 2)
}
