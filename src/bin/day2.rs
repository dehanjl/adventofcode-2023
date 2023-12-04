use adventofcode_2022::runner;
use hashbrown::HashMap;
use itertools::Itertools;

const MAX_R: u32 = 12;
const MAX_G: u32 = 13;
const MAX_B: u32 = 14;
type Colors = (u32, u32, u32);

fn parse_game(game: &str) -> Vec<Colors> {
    game.trim()
        .split(';')
        .map(|set| {
            let (mut r, mut g, mut b) = (0, 0, 0);
            set.trim().split(',').for_each(|s| {
                let (v, c) = sscanf::scanf!(s.trim(), "{u32} {str}").unwrap();
                match c {
                    "red" => r += v,
                    "green" => g += v,
                    "blue" => b += v,
                    _ => panic!("Invalid color"),
                }
            });

            (r, g, b)
        })
        .collect_vec()
}

fn parse_input(input: &str) -> HashMap<usize, Vec<Colors>> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(':').unwrap();
            let id = sscanf::scanf!(left, "Game {usize}").unwrap();
            let game = parse_game(right);
            (id, game)
        })
        .collect::<HashMap<usize, Vec<Colors>>>()
}

fn part1(input: &str) {
    let games = parse_input(input);
    let mut impossible_ids = Vec::new();

    'outer: for (id, game) in games.iter() {
        for (r, g, b) in game.iter() {
            if *r > MAX_R || *g > MAX_G || *b > MAX_B {
                impossible_ids.push(*id);
                continue 'outer;
            }
        }
    }

    let sum_game_ids = games.keys().sum::<usize>();

    println!(
        "Day 2 Part 1: {}",
        sum_game_ids - impossible_ids.iter().sum::<usize>()
    );
}

fn part2(input: &str) {
    let games = parse_input(input);
    let sum_powers = games
        .iter()
        .map(|(_, game)| {
            let max_r = game.iter().map(|&(r, _, _)| r).max().unwrap();
            let max_g = game.iter().map(|&(_, g, _)| g).max().unwrap();
            let max_b = game.iter().map(|&(_, _, b)| b).max().unwrap();
            max_r * max_g * max_b
        })
        .sum::<u32>();

    println!("Day 2 Part 2: {}", sum_powers);
}

fn main() {
    runner(part1);
    runner(part2);
}
