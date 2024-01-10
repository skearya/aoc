fn main() {
    let lines = include_str!("../input.txt").lines().collect::<Vec<_>>();

    if let Some(line) = lines.first() {
        let line_index = 0;
        let numbers: Vec<_> = line.char_indices().filter(|x| x.1 != '.').collect();

        let mut look_past = 0;

        for (index, char) in &numbers {
            if char.is_numeric() && !(index <= &look_past) {
                let mut number: Vec<u32> = Vec::new();
                number.push(char.to_digit(10).unwrap());

                let mut index2 = index + 1;

                while let Some(element) = numbers.iter().find(|x| x.0 == index2) {
                    number.push(element.1.to_digit(10).unwrap());
                    index2 += 1;
                }

                look_past = index + number.len() - 1;

                let (prev_char, next_char) =
                    (line.chars().nth(index - 1), line.chars().nth(look_past));

                let condition = |x: char| x != '.' && !x.is_numeric();

                if prev_char.is_some_and(condition) || next_char.is_some_and(condition) {
                    println!(
                        "{}: {}, {}",
                        char,
                        prev_char.unwrap_or('X'),
                        next_char.unwrap_or('X')
                    )
                }

                let (prev_line, next_line) = (lines.get(line_index - 1), lines.get(line_index + 1));
            }
        }
    }
}
