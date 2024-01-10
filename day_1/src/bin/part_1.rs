fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .split("\n")
            .map(|line| remove_non_digits(line))
            .sum::<u32>()
    );
}

fn remove_non_digits(line: &str) -> u32 {
    let filtered: Vec<u32> = line
        .chars()
        .filter_map(|character| character.to_digit(10))
        .collect();

    10 * filtered[0] + filtered[filtered.len() - 1]
}
