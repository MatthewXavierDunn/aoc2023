use std::{fmt::Debug, iter::repeat};

use crate::grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Galaxy,
}

pub fn part1(lines: impl Iterator<Item = String>) {
    let mut grid = lines.fold(Grid::new(), |mut grid, line| {
        let mut all_empty = true;
        let row = line
            .chars()
            .map(|char| match char {
                '.' => Tile::Empty,
                '#' => {
                    all_empty = false;
                    Tile::Galaxy
                }
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        if all_empty {
            grid.push_row(row.clone().into_iter());
        }
        grid.push_row(row.into_iter());
        grid
    });
    let mut col = 0;
    while col < grid.cols() {
        if grid.get_col(col).all(|tile| tile == &Tile::Empty) {
            grid.insert_col(col, repeat(Tile::Empty).take(grid.rows()));
            col += 1;
        }
        col += 1;
    }
    let galaxies = grid
        .inner()
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut galaxies, (i, tile)| {
            if tile == &Tile::Galaxy {
                galaxies.push((i / grid.cols(), i % grid.cols()));
            }
            galaxies
        });
    let mut total_dist = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            total_dist += galaxies[i].0.abs_diff(galaxies[j].0);
            total_dist += galaxies[i].1.abs_diff(galaxies[j].1);
        }
    }
    println!("{total_dist}");
}

pub fn part2(lines: impl Iterator<Item = String>) {
    const EXPAND_SIZE: usize = 1000000;
    let mut expand_rows = Vec::new();
    let grid = lines.enumerate().fold(Grid::new(), |mut grid, (r, line)| {
        let mut all_empty = true;
        let row = line
            .chars()
            .map(|char| match char {
                '.' => Tile::Empty,
                '#' => {
                    all_empty = false;
                    Tile::Galaxy
                }
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        if all_empty {
            expand_rows.push(r);
        }
        grid.push_row(row.into_iter());
        grid
    });
    let mut expand_cols = Vec::new();
    for col in 0..grid.cols() {
        if grid.get_col(col).all(|tile| tile == &Tile::Empty) {
            expand_cols.push(col);
        }
    }
    let galaxies = grid
        .inner()
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut galaxies, (i, tile)| {
            if tile == &Tile::Galaxy {
                galaxies.push((i / grid.cols(), i % grid.cols()));
            }
            galaxies
        });
    let mut total_dist = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let mut dist =
                galaxies[i].0.abs_diff(galaxies[j].0) + galaxies[i].1.abs_diff(galaxies[j].1);
            let row_range = galaxies[i].0..galaxies[j].0;
            let col_range = if galaxies[i].1 <= galaxies[j].1 {
                galaxies[i].1..galaxies[j].1
            } else {
                galaxies[j].1..galaxies[i].1
            };
            for row in &expand_rows {
                if row_range.contains(row) {
                    dist += EXPAND_SIZE - 1;
                }
            }
            for col in &expand_cols {
                if col_range.contains(col) {
                    dist += EXPAND_SIZE - 1;
                }
            }
            total_dist += dist;
        }
    }
    println!("{total_dist}");
}
