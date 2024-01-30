fn main() {
    let mut lines = include_str!("../input.txt").lines();

    let mut instructions = lines.next().unwrap().chars().cycle();

    let _ = lines.next();

    let nodes: Vec<(&str, (&str, &str))> = lines
        .map(|line| line.split_once(" = ").unwrap())
        .map(|line| {
            (
                line.0,
                line.1
                    .trim_matches(&['(', ')'] as &[char])
                    .split_once(", ")
                    .unwrap(),
            )
        })
        .collect();

    println!("steps: {}", traverse_node("AAA", &nodes, &mut instructions));
}

fn traverse_node(
    key: &str,
    nodes: &Vec<(&str, (&str, &str))>,
    instructions: &mut std::iter::Cycle<std::str::Chars<'_>>,
) -> i32 {
    let mut step = 0;
    let mut next = key;

    while let Some(node) = nodes.iter().find(|node| node.0 == next) {
        if node.0 == "ZZZ" {
            break;
        }

        next = if instructions.next().unwrap() == 'L' {
            (node.1).0
        } else {
            (node.1).1
        };

        step += 1;
    }

    step
}
