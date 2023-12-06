use adventofcode_2023::runner;
use itertools::Itertools;

fn parse(text: &str) -> Vec<u64> {
    let re = regex::Regex::new(r"\b\d+\b").unwrap();
    re.captures_iter(text)
        .map(|cap| cap.get(0).unwrap().as_str().parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn concat(numbers: &[u64]) -> u64 {
    numbers
        .iter()
        .map(|&n| n.to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    let (times, dists) = input.lines().collect_tuple().unwrap();
    (parse(times), parse(dists))
}

fn part1(input: &str) {
    let (times, dists) = parse_input(input);

    let res = times
        .iter()
        .zip(dists.iter())
        .map(|(time, dist)| {
            (0..=*time)
                .map(|t| t * (time - t))
                .filter(|t| t > dist)
                .count() as u64
        })
        .product::<u64>();

    println!("Day 6 Part 1: {}", res);
}

fn part2(input: &str) {
    let (times, dists) = parse_input(input);
    let (time, dist) = (concat(&times), concat(&dists));

    let res = (0..=time)
        .map(|t| t * (time - t))
        .filter(|&t| t > dist)
        .count() as u64;

    println!("Day 6 Part 2: {}", res);
}

fn main() {
    runner(part1);
    runner(part2);
}
