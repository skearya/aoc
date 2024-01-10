fn main() {
    let games = include_str!("../input.txt")
        .lines()
        .map(game_into_tuple)
        .filter(|game| check(&game.1))
        .map(|x| x.0)
        .sum::<u32>();

    println!("{:#?}", games);
}

// precondition: in Vec<"[count] [color]"> form (e.g. "4 red")
fn check(draws: &Vec<&str>) -> bool {
    const MAX_VALUES: [(&str, u32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

    for draw in draws {
        let tuple = draw.split_once(" ").unwrap();

        for pair in MAX_VALUES {
            if tuple.1 == pair.0 {
                if tuple.0.parse::<u32>().unwrap() > pair.1 {
                    return false;
                }
            }
        }
    }

    true
}

fn game_into_tuple(game: &str) -> (u32, Vec<&str>) {
    let game = game.split_once(": ").unwrap();

    (
        game.0[5..].parse().unwrap(),
        game.1
            .split(|character| character == ',' || character == ';')
            .map(|draw| draw.trim())
            .collect(),
    )
}
