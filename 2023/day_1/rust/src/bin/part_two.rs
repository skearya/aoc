fn main() {
    let input: u32 = include_str!("../../../input.txt")
        .lines()
        .map(transform)
        .sum();

    println!("sum: {}", input);
}

fn transform(line: &str) -> u32 {
    let numwords: [(&str, u32); 20] = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    let mut filtered: Vec<u32> = Vec::new();

    for i in 0..line.len() {
        for word in numwords {
            if line[i..].starts_with(word.0) {
                filtered.push(word.1)
            }
        }
    }

    filtered[0] * 10 + filtered[filtered.len() - 1]
}
