fn main() {
    let lines: Vec<&str> = include_str!("../input.txt").lines().collect();
    let mut numbers: Vec<((usize, usize), String)> = Vec::new();

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

                numbers.push(((line_idx, char_idx), number));
            }
        }
    }

    let mut gear_ratios: Vec<u32> = Vec::new();

    for (line_idx, line) in lines.iter().enumerate() {
        for (char_idx, char) in line.char_indices() {
            if !char.is_numeric() && char != '.' {
                let nearby_numbers: Vec<_> = numbers
                    .iter()
                    .filter(|num| {
                        let mut passed = false;

                        if (line_idx.saturating_sub(1)..=line_idx + 1).contains(&(num.0).0) {
                            for i in (num.0).1..(num.0).1 + num.1.len() {
                                if (char_idx.saturating_sub(1)..=char_idx + 1).contains(&i) {
                                    passed = true;
                                }
                            }
                        }

                        passed
                    })
                    .map(|t| t.1.parse::<u32>().unwrap())
                    .collect();

                if nearby_numbers.len() == 2 {
                    gear_ratios.push(nearby_numbers[0] * nearby_numbers[1])
                }
            }
        }
    }

    println!("{}", gear_ratios.iter().sum::<u32>());
}
