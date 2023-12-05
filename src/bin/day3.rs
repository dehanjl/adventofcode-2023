use adventofcode_2023::runner;
use hashbrown::HashSet;
use itertools::Itertools;

const RADIX: u32 = 10;
type Grid = Vec<Vec<char>>;
type Pos = (usize, usize);
type PartNum = Vec<(Pos, u32)>;

fn concat(vec: &[u32]) -> u32 {
    vec.iter().fold(0, |acc, &x| acc * 10 + x)
}

fn valid_neighbors(pos: Pos, grid: &Grid) -> Vec<Pos> {
    let y_max = grid.len();
    let x_max = grid[0].len();

    (-1..=1)
        .cartesian_product(-1..=1)
        .map(|(dx, dy)| (pos.0 as i32 + dx, pos.1 as i32 + dy))
        .filter_map(|(nx, ny)| {
            if nx < 0 || ny < 0 || nx >= x_max as i32 || ny >= y_max as i32 {
                return None;
            }
            Some((nx as usize, ny as usize))
        })
        .collect_vec()
}

fn is_adjacent_to_symbol(part: &PartNum, grid: &Grid) -> bool {
    part.iter()
        .flat_map(|((x, y), _)| {
            valid_neighbors((*x, *y), grid).into_iter().map(|(nx, ny)| {
                if grid[ny][nx].is_digit(RADIX) || grid[ny][nx] == '.' {
                    return false;
                }
                true
            })
        })
        .any(|x| x)
}

fn find_possible_part_nums(input: &str) -> Vec<PartNum> {
    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
    let mut possible_parts: Vec<PartNum> = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        let mut window: PartNum = Vec::new();
        for (x, c) in row.iter().enumerate() {
            if c.is_digit(RADIX) {
                let num = c.to_digit(RADIX).unwrap();
                window.push(((x, y), num));
            } else if !window.is_empty() {
                possible_parts.push(window.clone());
                window.clear();
            }
        }
        if !window.is_empty() {
            possible_parts.push(window.clone());
        }
    }

    possible_parts
}

fn part1(input: &str) {
    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
    let nums = find_possible_part_nums(input)
        .iter()
        .filter(|&part| is_adjacent_to_symbol(part, &grid))
        .map(|part| part.iter().map(|(_, num)| *num).collect_vec())
        .map(|vec| concat(&vec))
        .collect_vec();

    println!("Day 3 Part 1: {}", nums.iter().sum::<u32>());
}

fn part2(input: &str) {
    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();
    let possible_gears = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (x, y, c)))
        .filter(|(_, _, c)| **c == '*')
        .map(|(x, y, _)| (x, y))
        .collect_vec();
    let possible_parts = find_possible_part_nums(input);

    let mut ratios: Vec<u32> = Vec::new();
    for gear_pos in possible_gears {
        let mut gear_nums: HashSet<PartNum> = HashSet::new();
        for (x, y) in valid_neighbors(gear_pos, &grid) {
            if grid[y][x].is_digit(RADIX) {
                for part_num in &possible_parts {
                    if part_num.iter().any(|((px, py), _)| *px == x && *py == y) {
                        gear_nums.insert(part_num.clone());
                    }
                }
            }
        }
        if gear_nums.len() == 2 {
            let gear_ratio = gear_nums
                .iter()
                .map(|part| part.iter().map(|(_, num)| *num).collect_vec())
                .map(|vec| concat(&vec))
                .product();
            ratios.push(gear_ratio);
        }
    }

    println!("Day 3 Part 2: {}", ratios.iter().sum::<u32>());
}

fn main() {
    runner(part1);
    runner(part2);
}
