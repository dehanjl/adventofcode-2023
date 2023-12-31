use adventofcode_2023::runner;
use cached::proc_macro::cached;
use cached::SizedCache;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<(String, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(' ').unwrap();

            (
                String::from(l),
                r.split(',').map(|s| s.parse().unwrap()).collect_vec(),
            )
        })
        .collect_vec()
}

fn compress_binary(mut input: usize) -> usize {
    let mut output = 0;

    while input != 0 {
        let lead = input.leading_zeros();

        output <<= lead.min(1);
        output <<= 1;
        output |= 1;

        input <<= lead;
        input <<= 1;
    }

    output
}

#[cached(
    type = "SizedCache<String, u64>",
    create = "{ SizedCache::with_size(100) }",
    convert = r#"{ format!("{}{:?}", record, groups) }"#
)]
/// A port of https://www.reddit.com/r/adventofcode/comments/18hbbxe/2023_day_12python_stepbystep_tutorial_with_bonus/
fn calc(record: &str, groups: &[u64]) -> u64 {
    if groups.is_empty() {
        if !record.contains('#') {
            return 1;
        } else {
            return 0;
        }
    }

    if record.is_empty() {
        return 0;
    }

    let next_character = &record[0..1];
    let next_group = groups[0];

    let pound = || {
        let k = record.len().min(next_group as usize);
        let this_group = &record[..k];

        if this_group.replace('?', "#") != "#".repeat(next_group as usize) {
            return 0;
        }

        if record.len() == next_group as usize {
            if groups.len() == 1 {
                return 1;
            } else {
                return 0;
            }
        }

        if record[next_group as usize..].starts_with('#') {
            return 0;
        }

        calc(&record[next_group as usize + 1..], &groups[1..])
    };

    let dot = || calc(&record[1..], groups);

    match next_character {
        "#" => pound(),
        "." => dot(),
        "?" => dot() + pound(),
        _ => unreachable!(),
    }
}

fn part1(input: &str) {
    let mut sum = 0;
    for (s, v) in parse_input(input).iter() {
        let id = v.iter().fold(0, |x: usize, n| !(!x << n) << 1) >> 1;
        // mask to check for 1s ('#')
        let mask_a = s.chars().fold(0, |x: usize, c| match c {
            '#' => (x << 1) | 1,
            _ => x << 1,
        });

        // mask to check for 0s ('.')
        let mask_b = !s.chars().fold(0, |x: usize, c| match c {
            '#' | '?' => (x << 1) | 1,
            _ => x << 1,
        });

        let k = (0..2_usize.pow(s.len() as u32))
            .filter(|val| (val & mask_a) == mask_a)
            .filter(|val| (!val & mask_b) == mask_b)
            .map(compress_binary)
            .filter(|val| *val == id)
            .count();

        sum += k;
    }
    println!("Day 12 Part 1: {}", sum);
}

fn part2(input: &str) {
    let sum = parse_input(input)
        .iter()
        .map(|(l, r)| {
            let mut s = format!("{}{}", l, "?").repeat(4);
            s.push_str(l);
            let v = r.repeat(5);
            calc(&s, &v)
        })
        .sum::<u64>();
    println!("Day 12 Part 2: {}", sum);
}

fn main() {
    runner(part1);
    runner(part2);
}
