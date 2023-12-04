use std::cell::RefCell;

use adventofcode_2022::runner;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

fn parse(text: &str) -> Vec<u32> {
    let re = regex::Regex::new(r"\b\d{1,2}\b").unwrap();
    re.captures_iter(text)
        .map(|cap| cap.get(0).unwrap().as_str().parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn parse_input(input: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(':').unwrap().1.split_once('|').unwrap();
            (parse(left), parse(right))
        })
        .collect_vec()
}

fn intersection_count(a: &[u32], b: &[u32]) -> u32 {
    let a: HashSet<u32> = HashSet::from_iter(a.iter().cloned());
    let b: HashSet<u32> = HashSet::from_iter(b.iter().cloned());

    a.intersection(&b).count() as u32
}

fn part1(input: &str) {
    let points = parse_input(input)
        .iter()
        .map(|(l, r)| {
            let count = intersection_count(l, r);
            if count == 0 {
                0
            } else {
                u32::pow(2, count - 1)
            }
        })
        .sum::<u32>();

    println!("Day 4 Part 1: {}", points);
}

fn part2(input: &str) {
    // id, left, right, count
    let cards: HashMap<usize, (Vec<u32>, Vec<u32>, RefCell<usize>)> = parse_input(input)
        .iter()
        .enumerate()
        .map(|(id, (l, r))| (id + 1, (l.clone(), r.clone(), RefCell::new(1))))
        .collect();

    for (id, (l, r, count)) in cards.iter().sorted() {
        let offset = intersection_count(l, r) as usize;

        (id + 1..=id + offset).for_each(|idx| {
            *cards.get(&idx).unwrap().2.borrow_mut() += *count.borrow();
        });
    }

    let total_cards = cards
        .values()
        .map(|(_, _, count)| *count.borrow())
        .sum::<usize>();

    println!("Day 4 Part 2: {}", total_cards);
}

fn main() {
    runner(part1);
    runner(part2);
}
