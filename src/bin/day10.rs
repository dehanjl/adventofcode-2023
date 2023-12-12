use std::collections::VecDeque;

use adventofcode_2023::runner;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

type Pos = (usize, usize); // (x, y)
type Grid = Vec<Vec<Tile>>;

#[allow(dead_code)]
fn display_grid(grid: &Grid, path: &HashSet<Tile>) {
    for row in grid.iter() {
        for tile in row.iter() {
            let icon = if path.contains(tile) {
                tile.icon.to_string()
            } else if tile.is_enclosed(grid, path) {
                "\x1B[1;47mI\x1B[0m".to_string()
            } else {
                " ".to_string()
            };
            print!("{}", icon);
        }
        println!();
    }
}

fn pos_neighbors(pos: Pos, grid: &Grid) -> Vec<(Pos, Dir)> {
    let y_max = grid.len();
    let x_max = grid[0].len();

    [Dir::North, Dir::South, Dir::East, Dir::West]
        .iter()
        .map(|dir| (dir.step(), dir))
        .map(|((dx, dy), dir)| ((pos.0 as isize + dx, pos.1 as isize + dy), dir))
        .filter_map(|((nx, ny), dir)| {
            if nx < 0 || ny < 0 || nx >= x_max as isize || ny >= y_max as isize {
                return None;
            }
            Some(((nx as usize, ny as usize), *dir))
        })
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn step(&self) -> (isize, isize) {
        match self {
            Dir::North => (0, -1),
            Dir::South => (0, 1),
            Dir::East => (1, 0),
            Dir::West => (-1, 0),
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::East => Dir::West,
            Dir::West => Dir::East,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Tile {
    icon: char,
    pos: Pos,
    connections: Vec<Dir>,
}

impl Tile {
    fn neighbors(&self, grid: &Grid) -> Vec<Tile> {
        pos_neighbors(self.pos, grid)
            .iter()
            .filter(|(_, dir)| self.connections.contains(dir))
            .filter_map(|(pos, dir)| {
                let neighbor_tile = &grid[pos.1][pos.0];
                if neighbor_tile.connections.contains(&dir.opposite()) {
                    return Some(neighbor_tile.clone());
                }
                None
            })
            .collect()
    }

    fn is_enclosed(&self, grid: &Grid, path: &HashSet<Tile>) -> bool {
        if path.contains(self) {
            return false;
        }

        let up_west_ray_count = (0..self.pos.1)
            .map(|y| &grid[y][self.pos.0])
            .filter(|&tile| path.contains(tile) && tile.connections.contains(&Dir::East))
            .count();

        let up_east_ray_count = (0..self.pos.1)
            .map(|y| &grid[y][self.pos.0])
            .filter(|&tile| path.contains(tile) && tile.connections.contains(&Dir::West))
            .count();

        let down_west_ray_count = (self.pos.1 + 1..grid.len())
            .map(|y| &grid[y][self.pos.0])
            .filter(|&tile| path.contains(tile) && tile.connections.contains(&Dir::West))
            .count();

        let down_east_ray_count = (self.pos.1 + 1..grid.len())
            .map(|y| &grid[y][self.pos.0])
            .filter(|&tile| path.contains(tile) && tile.connections.contains(&Dir::East))
            .count();

        let left_north_ray_count = (0..self.pos.0)
            .map(|x| &grid[self.pos.1][x])
            .filter(|&tile| path.contains(tile) && tile.connections.contains(&Dir::North))
            .count();

        let left_south_ray_count = (0..self.pos.0)
            .map(|x| &grid[self.pos.1][x])
            .filter(|&tile| path.contains(tile) && tile.connections.contains(&Dir::South))
            .count();

        let right_north_ray_count = (self.pos.0 + 1..grid[0].len())
            .map(|x| &grid[self.pos.1][x])
            .filter(|&tile| path.contains(tile) && tile.connections.contains(&Dir::North))
            .count();

        let right_south_ray_count = (self.pos.0 + 1..grid[0].len())
            .map(|x| &grid[self.pos.1][x])
            .filter(|&tile| path.contains(tile) && tile.connections.contains(&Dir::South))
            .count();

        let counts = [
            up_west_ray_count,
            up_east_ray_count,
            down_west_ray_count,
            down_east_ray_count,
            left_north_ray_count,
            left_south_ray_count,
            right_north_ray_count,
            right_south_ray_count,
        ];

        if counts.iter().any(|&x| x == 0) {
            return false;
        }

        counts.iter().any(|&x| x % 2 == 1)
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.icon)
    }
}

impl std::convert::From<char> for Tile {
    fn from(c: char) -> Self {
        Tile {
            icon: c,
            pos: (0, 0),
            connections: match c {
                '|' => vec![Dir::North, Dir::South],
                '-' => vec![Dir::East, Dir::West],
                'L' => vec![Dir::North, Dir::East],
                'J' => vec![Dir::North, Dir::West],
                '7' => vec![Dir::South, Dir::West],
                'F' => vec![Dir::South, Dir::East],
                _ => vec![],
            },
        }
    }
}

fn infer_start_connections(start: Pos, grid: &mut Grid) {
    let start_connections = pos_neighbors(start, grid)
        .iter()
        .filter_map(|((x, y), dir)| {
            let tile = &grid[*y][*x];
            if tile.connections.contains(&dir.opposite()) {
                return Some(*dir);
            }
            None
        })
        .collect_vec();

    grid[start.1][start.0].connections = start_connections;
}

fn parse_input(input: &str) -> (Grid, Pos) {
    let mut start: Pos = (0, 0);

    let mut grid: Grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (x, y);
                    }
                    let mut t: Tile = c.into();
                    t.pos = (x, y);
                    t
                })
                .collect()
        })
        .collect();

    infer_start_connections(start, &mut grid);

    (grid, start)
}

fn part1(input: &str) {
    let (grid, start) = parse_input(input);

    let mut deq: VecDeque<(Tile, usize)> =
        VecDeque::from(vec![(grid[start.1][start.0].clone(), 0)]);

    let mut path: HashMap<Tile, usize> = HashMap::from([(grid[start.1][start.0].clone(), 0)]);

    while let Some((tile, count)) = deq.pop_front() {
        let neighbor_tiles = tile
            .neighbors(&grid)
            .into_iter()
            .filter_map(|tile| {
                if let Some(&prev_count) = path.get(&tile) {
                    if prev_count <= count + 1 {
                        return None;
                    }
                }
                Some((tile, count + 1))
            })
            .collect_vec();

        deq.extend(neighbor_tiles.clone());
        path.extend(neighbor_tiles.clone());
    }

    let max_count = path.values().max().unwrap();

    println!("Day 10 Part 1: {}", max_count);
}

fn part2(input: &str) {
    let (grid, start) = parse_input(input);

    let mut deq: VecDeque<Tile> = VecDeque::from([grid[start.1][start.0].clone()]);
    let mut path: HashSet<Tile> = HashSet::from([grid[start.1][start.0].clone()]);

    while let Some(tile) = deq.pop_front() {
        let neighbor_tiles = tile
            .neighbors(&grid)
            .into_iter()
            .filter(|tile| !path.contains(tile))
            .collect_vec();

        deq.extend(neighbor_tiles.clone());
        path.extend(neighbor_tiles.clone());
    }

    let enclosed_tiles = grid
        .iter()
        .flatten()
        .filter(|tile| tile.is_enclosed(&grid, &path))
        .count();

    // display_grid(&grid, &path);

    println!("Day 10 Part 2: {}", enclosed_tiles);
}

fn main() {
    runner(part1);
    runner(part2);
}
