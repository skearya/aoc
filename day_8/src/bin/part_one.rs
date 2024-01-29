fn main() {
    let mut lines = include_str!("../input.txt").lines();

    let mut instructions = lines.next().unwrap().chars().cycle();

    let _ = lines.next();

    let nodes: Vec<(&str, (&str, &str))> = lines
        .map(|line| line.split_once(" = ").unwrap())
        .map(|line| {
            (
                line.0,
                line.1.trim_matches(&['(', ')']).split_once(", ").unwrap(),
            )
        })
        .collect();

    let first = nodes.iter().find(|node| node.0 == "AAA").unwrap();

    let mut steps = 0;
    let mut next = if instructions.next().unwrap() == 'L' {
        first.1 .0
    } else {
        first.1 .1
    };

    loop {
        let node = nodes.iter().find(|x| x.0 == next).unwrap();

        steps += 1;

        if node.0 == "ZZZ" {
            break;
        }

        next = if instructions.next().unwrap() == 'L' {
            (node.1).0
        } else {
            (node.1).1
        };
    }

    println!("steps: {steps}");
}
