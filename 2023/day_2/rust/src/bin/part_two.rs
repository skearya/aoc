use std::collections::HashMap;

fn main() {
    let games: u32 = include_str!("../../../input.txt")
        .lines()
        .map(game_to_sets)
        .map(game_to_power)
        .sum();

    println!("{:#?}", games);
}

// precondition: in Vec<"[count] [color]"> form (e.g. "4 red")
fn game_to_power(sets: Vec<&str>) -> u32 {
    let mut recorded_max = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

    for game in sets {
        let tuple = game.split_once(" ").unwrap();
        let num = tuple.0.parse::<u32>().unwrap();
        let max = recorded_max.get_mut(tuple.1).unwrap();

        if *max < num {
            *max = num;
        }
    }

    recorded_max.values().product()
}

fn game_to_sets(game: &str) -> Vec<&str> {
    game.split_once(": ")
        .unwrap()
        .1
        .split(|character| character == ',' || character == ';')
        .map(|draw| draw.trim())
        .collect()
}
