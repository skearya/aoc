fn main() {
    let lines: Vec<&str> = include_str!("../../../input.txt").lines().collect();
    let mut numbers: Vec<u32> = Vec::new();

    for (line_idx, line) in lines.iter().enumerate() {
        let mut look_after = 0;

        for (char_idx, char) in line.char_indices() {
            if char.is_numeric() && char_idx >= look_after {
                let mut number = String::new();

                look_after = char_idx;
                while let Some(other_char) = line.chars().nth(look_after) {
                    if other_char.is_numeric() {
                        number.push(other_char);
                        look_after += 1;
                    } else {
                        break;
                    }
                }

                let l_r_chars = [
                    char_idx
                        .checked_sub(1)
                        .and_then(|idx| line.chars().nth(idx)),
                    line.chars().nth(look_after),
                ];

                let adjacent_lines = [
                    line_idx.checked_sub(1).and_then(|idx| lines.get(idx)),
                    lines.get(line_idx + 1),
                ];

                let indices = char_idx.saturating_sub(1)..look_after + 1;
                let adjacent_chars = adjacent_lines
                    .iter()
                    .flatten()
                    .flat_map(|line| indices.clone().flat_map(|i| line.chars().nth(i)));

                for char in adjacent_chars.chain(l_r_chars.into_iter().flatten()) {
                    if !char.is_numeric() && char != '.' {
                        numbers.push(number.parse().unwrap());
                    }
                }
            }
        }
    }

    println!("sum: {:?}", numbers.iter().sum::<u32>());
}
