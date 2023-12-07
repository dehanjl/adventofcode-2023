use adventofcode_2023::runner;
use hashbrown::HashMap;
use itertools::{sorted, Itertools};
use std::cmp::Ordering;

type Card = char;

const CARD_ORDER: [Card; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const CARD_ORDER_JOKE: [Card; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

trait Rank {
    fn rank(&self, order: &[Card]) -> usize;
}

impl Rank for Card {
    fn rank(&self, order: &[Card]) -> usize {
        order
            .iter()
            .position(|&x| x == *self)
            .expect("Card not found")
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand(String);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut counts: HashMap<Card, usize> = HashMap::new();
        for card in self.0.chars() {
            *counts.entry(card).or_insert(0) += 1;
        }

        let mut sorted_counts = sorted(counts.values()).rev();
        let first = sorted_counts.next().unwrap_or(&0);
        let second = sorted_counts.next().unwrap_or(&0);

        match (first, second) {
            (5, _) => HandType::FiveKind,
            (4, _) => HandType::FourKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeKind,
            (2, 2) => HandType::TwoPair,
            (2, _) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn get_type_joker(&self) -> HandType {
        CARD_ORDER_JOKE[0..]
            .iter()
            .map(|c| self.0.clone().replace('J', &c.to_string()))
            .map(|s| Hand(s).get_type())
            .min()
            .unwrap()
    }

    fn compare(&self, other: &Self, order: &[Card], joker: bool) -> Ordering {
        let (self_type, other_type) = if joker {
            (self.get_type_joker(), other.get_type_joker())
        } else {
            (self.get_type(), other.get_type())
        };

        if self_type != other_type {
            return (self_type as usize).cmp(&(other_type as usize)).reverse();
        } else {
            for i in 0..5 {
                let self_card = self.0.chars().nth(i).unwrap();
                let other_card = other.0.chars().nth(i).unwrap();
                if self_card != other_card {
                    return self_card.rank(order).cmp(&other_card.rank(order));
                }
            }
        }
        Ordering::Equal
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        Hand(String::from(value))
    }
}

fn parse_input(input: &str) -> Vec<(Hand, i64)> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            (Hand::from(hand), bid.parse::<i64>().unwrap())
        })
        .collect_vec()
}

fn part1(input: &str) {
    let mut hands = parse_input(input);
    hands.sort_by(|(hand_a, _), (hand_b, _)| hand_a.compare(hand_b, &CARD_ORDER, false));

    let winnings = hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i as i64 + 1))
        .sum::<i64>();

    println!("Day 7 Part 1: {}", winnings);
}

fn part2(input: &str) {
    let mut hands = parse_input(input);
    hands.sort_by(|(hand_a, _), (hand_b, _)| hand_a.compare(hand_b, &CARD_ORDER_JOKE, true));

    let winnings = hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i as i64 + 1))
        .sum::<i64>();

    println!("Day 7 Part 2: {}", winnings);
}

fn main() {
    runner(part1);
    runner(part2);
}
