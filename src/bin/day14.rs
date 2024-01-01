use adventofcode_2023::runner;
use grid::Grid;
use hashbrown::HashMap;
use itertools::Itertools;

const MAX_CYCLES: usize = 1_000_000_000;

#[allow(dead_code)]
pub fn display_grid(grid: &Grid<char>) {
    for row in grid.iter_rows() {
        for item in row {
            print!("{}", item);
        }
        println!();
    }
}

fn parse_input(input: &str) -> Grid<char> {
    let cols = input.lines().next().unwrap().len();
    let chars = input.lines().flat_map(|line| line.chars()).collect_vec();
    Grid::from_vec(chars, cols)
}

enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn adjust_idx(&self, row: usize, col: usize) -> (usize, usize) {
        match *self {
            Self::North => (row - 1, col),
            Self::West => (row, col - 1),
            Self::South => (row + 1, col),
            Self::East => (row, col + 1),
        }
    }
}

/// Finds the new pos of a rock if it is moved as far as possible in the given direction.
fn find_dest(
    mut row: usize,
    mut col: usize,
    grid: &Grid<char>,
    dir: Direction,
) -> Option<(usize, usize)> {
    (row, col) = dir.adjust_idx(row, col);

    let mut ret = None;
    while let Some(&c) = grid.get(row, col) {
        if c != '.' {
            break;
        };
        ret = Some((row, col));
        (row, col) = dir.adjust_idx(row, col);
    }

    ret
}

/// Rolls a single spot in the grid in the given direction. Only rolls it if it is round ('O').
fn roll(grid: &mut Grid<char>, row: usize, col: usize, dir: Direction) {
    if grid[(row, col)] == 'O' {
        if let Some((new_row, new_col)) = find_dest(row, col, grid, dir) {
            grid[(new_row, new_col)] = 'O';
            grid[(row, col)] = '.';
        }
    }
}

fn weight(grid: &Grid<char>) -> usize {
    grid.iter_rows()
        .enumerate()
        .map(|(i, row)| row.into_iter().filter(|&&item| item == 'O').count() * (grid.rows() - i))
        .sum::<usize>()
}

fn part1(input: &str) {
    let mut grid = parse_input(input);

    for i in 0..grid.rows() {
        for j in 0..grid.cols() {
            roll(&mut grid, i, j, Direction::North);
        }
    }

    println!("Day 14 Part 1: {}", weight(&grid));
}

fn roll_cycle(grid: &mut Grid<char>) {
    for i in 0..grid.rows() {
        for j in 0..grid.cols() {
            roll(grid, i, j, Direction::North);
        }
    }

    for j in 0..grid.cols() {
        for i in 0..grid.rows() {
            roll(grid, i, j, Direction::West);
        }
    }

    for i in (0..grid.rows()).rev() {
        for j in 0..grid.cols() {
            roll(grid, i, j, Direction::South);
        }
    }

    for j in (0..grid.cols()).rev() {
        for i in 0..grid.rows() {
            roll(grid, i, j, Direction::East);
        }
    }
}

fn part2(input: &str) {
    let mut grid = parse_input(input);

    let mut store: HashMap<Vec<char>, usize> = HashMap::new();
    let (mut cycle_start, mut cycle_len) = (usize::MAX, usize::MAX);
    for idx in 1..MAX_CYCLES {
        roll_cycle(&mut grid);
        let k = grid.clone().into_vec();
        if let Some(&n) = store.get(&k) {
            (cycle_start, cycle_len) = (n, idx - n);
            break;
        }
        store.insert(k, idx);
    }
    let n = (MAX_CYCLES - cycle_start) / cycle_len;
    let idx = MAX_CYCLES - (n * cycle_len);
    let cycled_grid = Grid::from_vec(
        store.iter().find(|(_, v)| **v == idx).unwrap().0.to_owned(),
        grid.cols(),
    );

    println!("Day 14 Part 2: {}", weight(&cycled_grid));
}

fn main() {
    runner(part1);
    runner(part2);
}
