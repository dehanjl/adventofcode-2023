use adventofcode_2022::runner;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

type Seed = i64;

#[derive(Debug)]
struct Map(Vec<(i64, i64, i64)>);

impl Map {
    fn map(&self, source: i64) -> i64 {
        let mut destination = source;

        for (drs, srs, l) in self.0.iter() {
            let offset = drs - srs;
            if source >= *srs && source < srs + l {
                destination = source + offset;
                break;
            }
        }

        destination
    }
}

fn parse_input(input: &str) -> (Vec<Seed>, Vec<Map>) {
    let blocks = input.split("\n\n").collect_vec();
    let seeds = blocks
        .first()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|s| s.parse::<Seed>().unwrap())
        .collect_vec();

    let maps = blocks
        .iter()
        .skip(1)
        .map(|block| {
            let map = block
                .lines()
                .skip(1)
                .map(|line| sscanf::scanf!(line, "{} {} {}", i64, i64, i64).unwrap())
                .collect_vec();

            Map(map)
        })
        .collect_vec();

    (seeds, maps)
}

fn map_through(maps: &[Map], seed: Seed) -> i64 {
    maps.iter().fold(seed, |acc, map| map.map(acc))
}

fn part1(input: &str) {
    let (seeds, maps) = parse_input(input);

    let loc = seeds
        .iter()
        .map(|&seed| map_through(&maps, seed))
        .min()
        .unwrap();

    println!("Day 5 Part 1: {}", loc);
}

fn part2(input: &str) {
    let (seeds, maps) = parse_input(input);

    let style = ProgressStyle::default_bar()
        .template("[{bar:40}] {percent}% {per_sec}")
        .unwrap()
        .progress_chars("#>-");

    let expanded_seeds = seeds
        .iter()
        .chunks(2)
        .into_iter()
        .map(|chunk| chunk.collect_tuple().unwrap())
        .map(|(start, len)| (*start..start + len))
        .flat_map(|it| it.clone())
        .collect_vec();

    let loc = expanded_seeds
        .par_iter()
        .progress_with_style(style)
        .map(|&seed| map_through(&maps, seed))
        .min()
        .unwrap();

    println!("Day 5 Part 2: {}", loc);
}

fn main() {
    runner(part1);
    runner(part2);
}
