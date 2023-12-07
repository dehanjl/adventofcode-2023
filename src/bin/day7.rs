use adventofcode_2023::runner;
use hashbrown::HashMap;
use itertools::{sorted, Itertools};
use std::cmp::Ordering;

type Card = char;

const CARD_ORDER: [Card; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

trait Rank {
    fn rank(&self) -> usize;
}

impl Rank for Card {
    fn rank(&self) -> usize {
        CARD_ORDER
            .iter()
            .position(|&x| x == *self)
            .expect("Card not found")
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand(String);

#[derive(Debug, PartialEq, Eq)]
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

        if *first == 5 {
            HandType::FiveKind
        } else if *first == 4 {
            HandType::FourKind
        } else if *first == 3 && *second == 2 {
            HandType::FullHouse
        } else if *first == 3 {
            HandType::ThreeKind
        } else if *first == 2 && *second == 2 {
            HandType::TwoPair
        } else if *first == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        Hand(String::from(value))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.get_type() != other.get_type() {
            return (self.get_type() as usize)
                .cmp(&(other.get_type() as usize))
                .reverse();
        } else {
            for i in 0..5 {
                let self_card = self.0.chars().nth(i).unwrap();
                let other_card = other.0.chars().nth(i).unwrap();
                if self_card != other_card {
                    return self_card.rank().cmp(&other_card.rank());
                }
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
    hands.sort_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b));

    println!("{:?}", hands);

    let winnings = hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i as i64 + 1))
        .sum::<i64>();

    println!("Day 7 Part 1: {}", winnings);
}

fn part2(input: &str) {}

fn main() {
    runner(part1);
    // runner(part2);
}
