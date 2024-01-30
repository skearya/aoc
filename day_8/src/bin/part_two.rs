use std::collections::HashMap;

fn main() {
    let mut lines = include_str!("../input.txt").lines();

    let instructions = lines.next().unwrap().chars().cycle();

    let _ = lines.next();

    let nodes: HashMap<&str, (&str, &str)> = lines
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

    let lcm = nodes
        .iter()
        .filter(|node| node.0.ends_with("A"))
        .map(|node| traverse_node(node.0, &nodes, instructions.clone()))
        .reduce(|x, acc| lcm(x, acc))
        .unwrap();

    println!("{:?}", lcm)
}

fn traverse_node(
    key: &str,
    nodes: &HashMap<&str, (&str, &str)>,
    mut instructions: std::iter::Cycle<std::str::Chars<'_>>,
) -> usize {
    let mut step = 0;
    let mut next = key;

    while let Some(node) = nodes.get_key_value(next) {
        if node.0.ends_with("Z") {
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

fn lcm(a: usize, b: usize) -> usize {
    return a * (b / gcd(a, b));
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}
