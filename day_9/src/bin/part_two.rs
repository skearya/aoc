fn main() {
    let sum = include_str!("../input.txt")
        .lines()
        .map(|line| line.split(" "))
        .map(|chars| {
            chars
                .flat_map(|num| num.parse::<i32>())
                .collect::<Vec<i32>>()
        })
        .map(next_element)
        .sum::<i32>();

    println!("sum: {:#?}", sum)
}

fn next_element(line: Vec<i32>) -> i32 {
    let mut differences: Vec<Vec<i32>> = Vec::new();

    let mut first_line_differences = Vec::new();

    for (i, value) in line.iter().enumerate() {
        if let Some(next) = line.get(i + 1) {
            first_line_differences.push(next - value)
        }
    }

    differences.push(first_line_differences);

    while !differences
        .last()
        .map(|diffs| diffs.iter().all(|x| *x == 0))
        .unwrap_or(false)
    {
        let mut last_line = differences.last().unwrap().iter().peekable();
        let mut line_differences = Vec::new();

        while let Some(value) = last_line.next() {
            if let Some(next) = last_line.peek() {
                line_differences.push(*next - value)
            }
        }

        differences.push(line_differences);
    }

    let mut first_elements = differences
        .iter()
        .flat_map(|diffs| diffs.first())
        .rev()
        .peekable();

    let mut acc = 0;

    while let Some(_) = first_elements.next() {
        if let Some(next) = first_elements.peek() {
            acc = *next - acc;
        }
    }

    line.first().unwrap() - acc
}
