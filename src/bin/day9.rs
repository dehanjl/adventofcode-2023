use adventofcode_2023::runner;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect()
}

fn part1(input: &str) {
    let sequences = parse_input(input);
    let res = sequences
        .iter()
        .map(|orig_seq| {
            let mut seq = orig_seq.clone();
            let mut intermediate_seqs = vec![];
            loop {
                let diffs = seq.iter().tuple_windows().map(|(a, b)| b - a).collect_vec();
                if diffs.iter().all(|&x| x == 0) {
                    break;
                }

                intermediate_seqs.push(diffs.clone());
                seq = diffs;
            }
            let final_diff = intermediate_seqs
                .iter()
                .map(|v| v.last().unwrap())
                .sum::<i32>();
            (orig_seq, final_diff)
        })
        .map(|(seq, diff)| seq.last().unwrap() + diff)
        .sum::<i32>();

    println!("Day 9 Part 1: {}", res);
}

fn part2(input: &str) {
    let sequences = parse_input(input);
    let res = sequences
        .iter()
        .map(|orig_seq| {
            let mut seq = orig_seq.clone();
            let mut intermediate_seqs = vec![];
            loop {
                let diffs = seq.iter().tuple_windows().map(|(a, b)| b - a).collect_vec();
                if diffs.iter().all(|&x| x == 0) {
                    break;
                }

                intermediate_seqs.push(diffs.clone());
                seq = diffs;
            }
            let first_diff = intermediate_seqs
                .iter()
                .rev()
                .map(|v| v.first().unwrap())
                .fold(0, |acc, &x| x - acc);
            (orig_seq, first_diff)
        })
        .map(|(seq, diff)| seq.first().unwrap() - diff)
        .sum::<i32>();

    println!("Day 9 Part 2: {}", res);
}

fn main() {
    runner(part1);
    runner(part2);
}
