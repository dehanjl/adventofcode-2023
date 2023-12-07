use adventofcode_2023::runner;
use fnv::FnvHashMap;
use itertools::Itertools;

type Card = char;

#[derive(Debug, PartialEq, Eq)]
struct Hand(String);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveKind = 7_000_000,
    FourKind = 6_000_000,
    FullHouse = 5_000_000,
    ThreeKind = 4_000_000,
    TwoPair = 3_000_000,
    OnePair = 2_000_000,
    HighCard = 1_000_000,
}

impl Hand {
    fn high_pair(counts: &FnvHashMap<Card, usize>) -> (usize, usize) {
        let (mut first, mut second) = (0, 0);
        for &count in counts.values() {
            if count > first {
                second = first;
                first = count;
            } else if count > second {
                second = count;
            }
        }
        (first, second)
    }

    fn match_pair(first: usize, second: usize) -> HandType {
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

    fn get_type(&self) -> HandType {
        let mut counts = FnvHashMap::with_capacity_and_hasher(5, Default::default());
        for card in self.0.chars() {
            *counts.entry(card).or_insert(0) += 1;
        }

        let (first, second) = Hand::high_pair(&counts);
        Hand::match_pair(first, second)
    }

    fn score(&self) -> u32 {
        self.get_type() as u32 + u32::from_str_radix(&self.0, 16).unwrap()
    }

    fn get_type_joker(&self) -> HandType {
        let mut counts = FnvHashMap::with_capacity_and_hasher(5, Default::default());
        for card in self.0.chars() {
            *counts.entry(card).or_insert(0) += 1;
        }

        let joker_count = counts.remove(&'1').unwrap_or(0);

        let (first, second) = Hand::high_pair(&counts);
        Hand::match_pair(first + joker_count, second)
    }

    fn score_joker(&self) -> u32 {
        self.get_type_joker() as u32 + u32::from_str_radix(&self.0, 16).unwrap()
    }
}

fn parse_input(input: &str) -> Vec<(Hand, u32)> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            (Hand(hand.to_string()), bid.parse::<u32>().unwrap())
        })
        .collect_vec()
}

fn part1(input: &str) {
    let replaced = input
        .replace('A', "E")
        .replace('K', "D")
        .replace('Q', "C")
        .replace('J', "B")
        .replace('T', "A");
    let mut hands = parse_input(&replaced);
    hands.sort_by_cached_key(|(hand, _)| hand.score());

    let winnings = hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i as u32 + 1))
        .sum::<u32>();

    println!("Day 7 Part 1: {}", winnings);
}

fn part2(input: &str) {
    let replaced = input
        .replace('A', "E")
        .replace('K', "D")
        .replace('Q', "C")
        .replace('J', "1")
        .replace('T', "A");
    let mut hands = parse_input(&replaced);
    hands.sort_by_cached_key(|(hand, _)| hand.score_joker());

    let winnings = hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i as u32 + 1))
        .sum::<u32>();

    println!("Day 7 Part 2: {}", winnings);
}

fn main() {
    runner(part1);
    runner(part2);
}
