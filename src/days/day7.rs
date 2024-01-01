use itertools::Itertools;

use crate::utils::input_process::input_to_lines;
use core::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Eq, Clone)]
struct Cards {
    hand: String,
    bid: i32,
    name: HandNames,
}

impl Ord for Cards {
    fn cmp(&self, other: &Self) -> Ordering {
        let first_ordering = self.name.get_value().cmp(&other.name.get_value());

        if first_ordering == Ordering::Equal {
            for (ch, ch_other) in self.hand.chars().zip(other.hand.chars()) {
                let char_criteria = get_card_value(ch).cmp(&get_card_value(ch_other));
                if char_criteria != Ordering::Equal {
                    return char_criteria;
                }
            }
        }
        first_ordering
    }
}

impl PartialOrd for Cards {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cards {
    fn eq(&self, other: &Self) -> bool {
        self.name.get_value() == other.name.get_value() && self.hand == other.hand
    }
}

impl Cards {
    fn new(hand: String, bid: i32) -> Cards {
        let mut sets = HashMap::new();
        for ch in hand.chars() {
            *sets.entry(ch).or_insert(0) += 1;
        }

        // Process the joker
        let j_number = sets.get(&'J').unwrap_or(&0);

        if j_number > &0 {
            let mut new_hand = hand.clone();
            let biggest_set = sets
                .iter()
                .filter(|(key, _value)| *key != &'J')
                .max_by_key(|entry| entry.1)
                .unwrap_or((&'A', &5))
                .0;

            new_hand = new_hand.replace('J', &biggest_set.to_string());
            sets.clear();

            for ch in new_hand.chars() {
                *sets.entry(ch).or_insert(0) += 1;
            }
        }

        let name = get_name(&sets);

        Cards { hand, bid, name }
    }
}

fn get_card_value(ch: char) -> i32 {
    match ch {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => {
            panic!("Err: unmanaged char {ch}")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum HandNames {
    Nil,
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl HandNames {
    fn get_value(&self) -> i32 {
        match self {
            HandNames::Nil => 0,
            HandNames::HighCard => 1,
            HandNames::OnePair => 2,
            HandNames::TwoPair => 3,
            HandNames::ThreeKind => 4,
            HandNames::FullHouse => 5,
            HandNames::FourKind => 6,
            HandNames::FiveKind => 7,
        }
    }
}

fn get_name(sets: &HashMap<char, i32>) -> HandNames {
    let mut set_counts: HashMap<i32, i32> = HashMap::new();

    for value in sets.values() {
        *set_counts.entry(*value).or_insert(0) += 1;
    }

    if set_counts.get(&5).is_some() {
        return HandNames::FiveKind;
    }

    if set_counts.get(&4).is_some() {
        return HandNames::FourKind;
    }

    if set_counts.get(&3).is_some() {
        if set_counts.get(&2).is_some() {
            return HandNames::FullHouse;
        } else {
            return HandNames::ThreeKind;
        }
    }

    if let Some(counts) = set_counts.get(&2) {
        if counts == &2 {
            return HandNames::TwoPair;
        } else {
            return HandNames::OnePair;
        }
    };

    if let Some(counts) = set_counts.get(&1) {
        if counts == &5 {
            return HandNames::HighCard;
        }
    }

    HandNames::Nil
}

fn format_data(lines: &[String]) -> impl Iterator<Item = Cards> + '_ {
    lines.iter().map(|line| {
        let parts = line.split(' ').collect::<Vec<&str>>();
        Cards::new(
            parts[0].to_string(),
            parts[1].parse::<i32>().expect("Err: could not parse i32"),
        )
    })
}

fn process_cards(cards: impl Iterator<Item = Cards>) -> i32 {
    cards
        .sorted()
        .enumerate()
        .map(|(i, card)| (i as i32 + 1) * card.bid)
        .sum::<i32>()
}

fn day7(input: &str) -> i32 {
    let data = input_to_lines(input);
    let cards = format_data(&data);
    process_cards(cards)
}

pub fn run() {
    let input = "./days/day7/input.txt";
    let result = day7(input);
    println!("\n day7 done with result_part_one {result} ");
}
