use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct Card {
    num: u32,
    matching_nums: u32,
}

fn main() {
    let mut cards = include_str!("../input.txt")
        .lines()
        .map(parse_line)
        .collect::<Vec<Card>>();

    let starting_cards = cards.clone();

    for card in &starting_cards {
        for _ in 0..cards.iter().filter(|c| card.num == c.num).count() {
            for i in (card.num + 1)..=card.num + card.matching_nums {
                let copied_card = starting_cards
                    .iter()
                    .find(|card| card.num == i)
                    .unwrap()
                    .clone();
                cards.push(copied_card);
            }
        }
    }

    println!("amount of cards: {}", cards.len());
}

fn parse_line(line: &str) -> Card {
    let (card, numbers) = line.split_once(": ").unwrap();

    Card {
        num: card
            .split_whitespace()
            .last()
            .and_then(|num| num.parse::<u32>().ok())
            .unwrap(),
        matching_nums: winning_numbers_amount(numbers),
    }
}

fn winning_numbers_amount(line: &str) -> u32 {
    let (winning_numbers, mut numbers) = {
        let mut table = line.split(" | ").map(|list| {
            list.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u32>>()
        });

        (table.next().unwrap(), table.next().unwrap())
    };

    numbers.retain(|n| winning_numbers.contains(n));

    numbers.len().try_into().unwrap()
}
