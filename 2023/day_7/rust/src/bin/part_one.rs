use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Clone, Copy)]
enum Hand {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn main() {
    let mut hands: Vec<(Hand, &str, &str)> = include_str!("../../../input.txt")
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(get_type)
        .collect();

    hands.sort_by(|a, b| {
        let (type_a, type_b) = (a.0 as usize, b.0 as usize);

        if type_a > type_b {
            Ordering::Greater
        } else if type_a < type_b {
            Ordering::Less
        } else {
            let cards =
                a.1.chars()
                    .zip(b.1.chars())
                    .map(|(a, b)| (card_to_strength(a), card_to_strength(b)));

            for (card_a, card_b) in cards {
                if card_a < card_b {
                    return Ordering::Less;
                } else if card_a > card_b {
                    return Ordering::Greater;
                } else {
                    continue;
                }
            }

            Ordering::Equal
        }
    });

    println!(
        "sum: {}",
        hands
            .iter()
            .enumerate()
            .map(|(x, y)| (x + 1) * y.2.parse::<usize>().unwrap())
            .sum::<usize>()
    )
}

fn get_type<'a>((cards, bid): (&'a str, &'a str)) -> (Hand, &'a str, &'a str) {
    let mut map: HashMap<char, u16> = HashMap::new();

    for card in cards.chars() {
        *map.entry(card).or_insert(0) += 1;
    }

    let largest_kind = map.values().fold(0, |acc, num| acc.max(*num));
    let len = map.keys().len();

    let hand_type = match largest_kind {
        5 => Hand::FiveOfAKind,
        4 => Hand::FourOfAKind,
        3 => {
            if len == 2 {
                Hand::FullHouse
            } else {
                Hand::ThreeOfAKind
            }
        }
        2 => {
            if len == 3 {
                Hand::TwoPair
            } else {
                Hand::OnePair
            }
        }
        _ => Hand::HighCard,
    };

    (hand_type, cards, bid)
}

fn card_to_strength(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap(),
    }
}
