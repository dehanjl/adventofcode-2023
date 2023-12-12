use std::sync::OnceLock;

use adventofcode_2023::runner;
use grid::Grid;
use hashbrown::HashMap;
use itertools::Itertools;
use priority_queue::DoublePriorityQueue;
use rayon::iter::{ParallelBridge, ParallelIterator};

#[allow(dead_code)]
fn display_grid(grid: &Grid<char>) {
    for row in grid.iter_rows() {
        for item in row {
            print!("{}", item);
        }
        println!();
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Loc {
    x: isize,
    y: isize,
}

impl Loc {
    fn neighbors(&self, grid: &Grid<char>) -> Vec<Loc> {
        // down, right, up, left
        const DIRMAP: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        DIRMAP
            .iter()
            .map(|(dx, dy)| (self.x + dx, self.y + dy))
            .filter_map(|(x, y)| {
                if grid.get(y as usize, x as usize).is_some() {
                    return Some(Loc { x, y });
                }
                None
            })
            .collect_vec()
    }
}

fn parse_input(input: &str) -> Grid<char> {
    let cols = input.lines().next().unwrap().len();
    let chars = input.lines().flat_map(|line| line.chars()).collect_vec();
    Grid::from_vec(chars, cols)
}

fn empty_indices(grid: &Grid<char>) -> (Vec<usize>, Vec<usize>) {
    let empty_rows = grid
        .iter_rows()
        .enumerate()
        .filter_map(|(id, row)| {
            if row.into_iter().all(|&c| c == '.') {
                Some(id)
            } else {
                None
            }
        })
        .collect_vec();

    let empty_cols = grid
        .iter_cols()
        .enumerate()
        .filter_map(|(id, col)| {
            if col.into_iter().all(|&c| c == '.') {
                Some(id)
            } else {
                None
            }
        })
        .collect_vec();

    (empty_rows, empty_cols)
}

static EMPTY_INDICES: OnceLock<(Vec<usize>, Vec<usize>)> = OnceLock::new();

fn a_star(start: &Loc, end: &Loc, grid: &Grid<char>, factor: i64) -> i64 {
    /// Manhattan distance heuristic function.
    /// Because we can't step diagonally, this is admissable.
    fn h(loc: &Loc, end: &Loc) -> i64 {
        let dx = (loc.x - end.x).abs();
        let dy = (loc.y - end.y).abs();
        (dx + dy) as i64
    }

    let (empty_rows, empty_cols) = EMPTY_INDICES.get_or_init(|| empty_indices(grid));

    let mut open_set: DoublePriorityQueue<Loc, i64> = DoublePriorityQueue::new();
    let mut g_scores: HashMap<Loc, i64> = HashMap::new();
    g_scores.insert(*start, 0);
    let mut f_scores: HashMap<Loc, i64> = HashMap::new();
    f_scores.insert(*start, h(start, end));

    open_set.push(*start, *f_scores.get(start).unwrap());

    while !open_set.is_empty() {
        let current = open_set.pop_min().unwrap().0;
        if current == *end {
            return *g_scores.get(&current).unwrap();
        }

        for neighbor in current.neighbors(grid) {
            let mut tentative_g_score = g_scores[&current] + 1;
            if empty_rows.contains(&(neighbor.y as usize))
                || empty_cols.contains(&(neighbor.x as usize))
            {
                tentative_g_score += factor - 1;
            }

            if tentative_g_score < *g_scores.get(&neighbor).unwrap_or(&i64::MAX) {
                g_scores.insert(neighbor, tentative_g_score);
                f_scores.insert(neighbor, tentative_g_score + h(&neighbor, end));
                open_set.push(neighbor, *f_scores.get(&neighbor).unwrap());
            }
        }
    }

    i64::MAX
}

fn compute_distances(grid: &mut Grid<char>, factor: i64) -> Vec<i64> {
    let galaxies = grid
        .iter_rows()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter().enumerate().filter_map(move |(x, &c)| {
                if c == '#' {
                    Some(Loc {
                        x: x as isize,
                        y: y as isize,
                    })
                } else {
                    None
                }
            })
        })
        .collect_vec();

    galaxies
        .iter()
        .tuple_combinations()
        .par_bridge()
        .map(|(a, b)| a_star(a, b, grid, factor))
        .collect()
}

fn part1(input: &str) {
    let mut grid = parse_input(input);
    let distances: Vec<i64> = compute_distances(&mut grid, 2);
    println!("Day 11 Part 1: {}", distances.iter().sum::<i64>());
}

fn part2(input: &str) {
    let mut grid = parse_input(input);
    let distances: Vec<i64> = compute_distances(&mut grid, 1_000_000);
    println!("Day 11 Part 1: {}", distances.iter().sum::<i64>());
}

fn main() {
    runner(part1);
    runner(part2);
}
