use std::collections::HashSet;

use crate::grid::Grid;

pub fn part1(lines: impl Iterator<Item = String>) {
    let mut grid = Grid::new();
    lines.for_each(|line| {
        grid.push_row(line.chars());
    });
    println!("{}", energize(Dir::Right, 0, 0, &grid));
}

pub fn part2(lines: impl Iterator<Item = String>) {
    let mut grid = Grid::new();
    lines.for_each(|line| {
        grid.push_row(line.chars());
    });
    let mut max = 0;
    for r in 0..grid.rows() {
        max = max.max(energize(Dir::Right, r, 0, &grid));
        max = max.max(energize(Dir::Left, r, grid.cols() - 1, &grid));
    }
    for c in 0..grid.cols() {
        max = max.max(energize(Dir::Down, 0, c, &grid));
        max = max.max(energize(Dir::Up, grid.rows() - 1, c, &grid));
    }
    println!("{max}");
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn energize(dir: Dir, row: usize, col: usize, grid: &Grid<char>) -> usize {
    let mut beams = vec![(dir, row, col)];
    let mut seen = HashSet::new();
    while let Some((mut dir, mut r, mut c)) = beams.pop() {
        if seen.contains(&(dir, r, c)) {
            continue;
        }
        seen.insert((dir, r, c));
        match grid.get(r, c).unwrap() {
            '.' => (),
            '\\' => match dir {
                Dir::Up => dir = Dir::Left,
                Dir::Down => dir = Dir::Right,
                Dir::Left => dir = Dir::Up,
                Dir::Right => dir = Dir::Down,
            },
            '/' => match dir {
                Dir::Up => dir = Dir::Right,
                Dir::Down => dir = Dir::Left,
                Dir::Left => dir = Dir::Down,
                Dir::Right => dir = Dir::Up,
            },
            '-' => match dir {
                Dir::Up | Dir::Down => {
                    beams.push((Dir::Left, r, c));
                    dir = Dir::Right;
                }
                Dir::Left | Dir::Right => (),
            },
            '|' => match dir {
                Dir::Up | Dir::Down => (),
                Dir::Left | Dir::Right => {
                    beams.push((Dir::Up, r, c));
                    dir = Dir::Down;
                }
            },
            _ => unreachable!(),
        }
        match dir {
            Dir::Up => {
                if r == 0 {
                    continue;
                }
                r -= 1
            }
            Dir::Down => {
                if r == grid.rows() - 1 {
                    continue;
                }
                r += 1
            }
            Dir::Left => {
                if c == 0 {
                    continue;
                }
                c -= 1
            }
            Dir::Right => {
                if c == grid.cols() - 1 {
                    continue;
                }
                c += 1
            }
        }
        beams.push((dir, r, c));
    }
    let mut visited = HashSet::new();
    for (_, r, c) in seen {
        visited.insert((r, c));
    }
    visited.len()
}
