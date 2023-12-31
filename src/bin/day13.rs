use adventofcode_2023::runner;
use grid::Grid;
use itertools::Itertools;

#[allow(dead_code)]
fn display_grid(grid: &Grid<char>) {
    for row in grid.iter_rows() {
        for item in row {
            print!("{}", item);
        }
        println!();
    }
}

fn parse_input(input: &str) -> Vec<Grid<char>> {
    input
        .split("\n\n")
        .map(|s| {
            let cols = s.lines().next().unwrap().len();
            let chars = s.lines().flat_map(|line| line.chars()).collect_vec();
            Grid::from_vec(chars, cols)
        })
        .collect_vec()
}

fn get_reflection_axis(v: &Vec<Vec<&char>>) -> Vec<usize> {
    let mut axis = Vec::new();
    for i in 1..v.len() {
        let l_min = ((i - (v.len() - i)) as isize).max(0) as usize;
        let l = v[l_min..i].iter().collect_vec();
        let r_max = v.len().min(i + l.len());
        let r = v[i..r_max].iter().rev().collect_vec();

        if l == r {
            axis.push(i);
        }
    }
    axis
}

fn part1(input: &str) {
    let mut sum = 0;
    for grid in parse_input(input).iter() {
        let hor = grid.iter_rows().map(|v| v.collect_vec()).collect_vec();
        let ver = grid.iter_cols().map(|v| v.collect_vec()).collect_vec();

        sum += get_reflection_axis(&hor).first().unwrap_or(&0) * 100;
        sum += get_reflection_axis(&ver).first().unwrap_or(&0);
    }
    println!("Day 13 Part 1: {}", sum);
}

fn part2(input: &str) {
    let mut sum = 0;
    for grid in parse_input(input).iter() {
        let hor = grid.iter_rows().map(|v| v.collect_vec()).collect_vec();
        let ver = grid.iter_cols().map(|v| v.collect_vec()).collect_vec();

        let mut val = 0;
        if let Some(h) = get_reflection_axis(&hor).first() {
            val = *h * 100;
        }
        if let Some(v) = get_reflection_axis(&ver).first() {
            val = *v;
        }

        'inner: for (i, j) in (0..grid.rows()).cartesian_product(0..grid.cols()) {
            let mut g = grid.clone();
            let k = g.get_mut(i, j).unwrap();
            if k == &'#' {
                *k = '.';
            } else {
                *k = '#';
            }

            let hor_g = g.iter_rows().map(|v| v.collect_vec()).collect_vec();
            let ver_g = g.iter_cols().map(|v| v.collect_vec()).collect_vec();

            for h in get_reflection_axis(&hor_g) {
                if val != h * 100 {
                    val = h * 100;
                    break 'inner;
                }
            }

            for v in get_reflection_axis(&ver_g) {
                if val != v {
                    val = v;
                    break 'inner;
                }
            }
        }

        sum += val;
    }
    println!("Day 13 Part 2: {}", sum);
}

fn main() {
    runner(part1);
    runner(part2);
}
