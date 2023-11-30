use adventofcode_2022::runner;

fn parse_input(input: &str) -> Vec<u32> {
    let mut cal: u32 = 0;
    let mut cal_vec: Vec<u32> = Vec::new();
    for line in input.lines() {
        if !line.is_empty() {
            cal += line.parse::<u32>().unwrap();
        } else {
            cal_vec.push(cal);
            cal = 0;
        }
    }

    cal_vec
}

fn part1(input: &str) {
    println!("Day 1 Part 1: {}", parse_input(input).iter().max().unwrap());
}

fn part2(input: &str) {
    let mut cal_vec = parse_input(input);
    cal_vec.sort();
    println!(
        "Day 1 Part 2: {}",
        cal_vec.iter().rev().take(3).sum::<u32>()
    );
}

fn main() {
    runner(part1);
    runner(part2);
}
