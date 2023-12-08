use adventofcode_2023::runner;
use hashbrown::HashMap;
use itertools::Itertools;
use num::integer::lcm;

fn parse_input(input: &str) -> (String, HashMap<String, (String, String)>) {
    let (steps, map) = input.split_once("\n\n").unwrap();

    let nodes = map
        .lines()
        .map(|line| sscanf::scanf!(line, "{str} = ({str}, {str})").unwrap())
        .map(|(a, b, c)| (a.to_owned(), (b.to_owned(), c.to_owned())))
        .collect::<HashMap<_, _>>();

    (String::from(steps), nodes)
}

fn part1(input: &str) {
    let (steps, nodes) = parse_input(input);

    let mut curr_node = "AAA";
    let mut curr_steps = 0;

    loop {
        if curr_node == "ZZZ" {
            println!("Day 8 Part 1: {}", curr_steps);
            break;
        }

        let side = steps.chars().nth(curr_steps % steps.len()).unwrap();
        curr_node = if side == 'L' {
            nodes.get(curr_node).unwrap().0.as_str()
        } else {
            nodes.get(curr_node).unwrap().1.as_str()
        };

        curr_steps += 1;
    }
}

fn part2(input: &str) {
    let (steps, nodes) = parse_input(input);

    let start_nodes = nodes.keys().filter(|k| k.ends_with('A')).collect_vec();

    let lcm = start_nodes
        .iter()
        .map(|&node| {
            let mut curr_node = node;
            let mut curr_steps = 0;

            loop {
                if curr_node.ends_with('Z') {
                    break;
                }

                let side = steps.chars().nth(curr_steps % steps.len()).unwrap();
                curr_node = if side == 'L' {
                    &nodes.get(curr_node).unwrap().0
                } else {
                    &nodes.get(curr_node).unwrap().1
                };

                curr_steps += 1;
            }

            curr_steps as u64
        })
        .fold(1, lcm);

    println!("Day 8 Part 2: {}", lcm);
}

fn main() {
    runner(part1);
    runner(part2);
}
