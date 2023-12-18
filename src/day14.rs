use std::collections::HashMap;

use crate::grid::Grid;

pub fn tilt_north(grid: &mut Grid<char>) {
    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            if grid.get(r, c).unwrap() == &'O' {
                grid.set(r, c, '.');
                let mut nr = r as isize;
                loop {
                    if nr < 0 {
                        break;
                    }
                    let elem = grid.get(nr as usize, c).unwrap();
                    if elem == &'.' {
                        nr -= 1;
                    } else {
                        break;
                    }
                }
                grid.set((nr + 1) as usize, c, 'O');
            }
        }
    }
}

pub fn tilt_south(grid: &mut Grid<char>) {
    for r in (0..grid.rows()).rev() {
        for c in 0..grid.cols() {
            if grid.get(r, c).unwrap() == &'O' {
                grid.set(r, c, '.');
                let mut nr = r;
                loop {
                    if nr >= grid.cols() {
                        break;
                    }
                    let elem = grid.get(nr, c).unwrap();
                    if elem == &'.' {
                        nr += 1;
                    } else {
                        break;
                    }
                }
                grid.set(nr - 1, c, 'O');
            }
        }
    }
}

pub fn tilt_west(grid: &mut Grid<char>) {
    for c in 0..grid.cols() {
        for r in 0..grid.rows() {
            if grid.get(r, c).unwrap() == &'O' {
                grid.set(r, c, '.');
                let mut nc = c as isize;
                loop {
                    if nc < 0 {
                        break;
                    }
                    let elem = grid.get(r, nc as usize).unwrap();
                    if elem == &'.' {
                        nc -= 1;
                    } else {
                        break;
                    }
                }
                grid.set(r, (nc + 1) as usize, 'O');
            }
        }
    }
}

pub fn tilt_east(grid: &mut Grid<char>) {
    for c in (0..grid.cols()).rev() {
        for r in 0..grid.rows() {
            if grid.get(r, c).unwrap() == &'O' {
                grid.set(r, c, '.');
                let mut nc = c;
                loop {
                    if nc >= grid.cols() {
                        break;
                    }
                    let elem = grid.get(r, nc).unwrap();
                    if elem == &'.' {
                        nc += 1;
                    } else {
                        break;
                    }
                }
                grid.set(r, nc - 1, 'O');
            }
        }
    }
}

fn cycle(grid: &mut Grid<char>) {
    tilt_north(grid);
    tilt_west(grid);
    tilt_south(grid);
    tilt_east(grid);
}

pub fn part1(lines: impl Iterator<Item = String>) {
    let mut grid = Grid::new();
    lines.for_each(|line| {
        grid.push_row(line.chars());
    });
    tilt_north(&mut grid);
    let mut sum = 0;
    for r in 0..grid.rows() {
        for item in grid.get_row(r) {
            if item == &'O' {
                sum += grid.rows() - r;
            }
        }
    }
    println!("{sum}");
}

pub fn part2(lines: impl Iterator<Item = String>) {
    const CYCLES: usize = 1000000000;
    let mut grid = Grid::new();
    lines.for_each(|line| {
        grid.push_row(line.chars());
    });
    let mut seen = HashMap::new();
    let mut i = 0;
    while i < CYCLES {
        cycle(&mut grid);

        if seen.contains_key(&grid) {
            let n = seen.get(&grid).unwrap();
            let loop_len = i - n;
            i = CYCLES - (CYCLES - i) % loop_len;
        }
        seen.insert(grid.clone(), i);
        i += 1;
    }
    let mut sum = 0;
    for r in 0..grid.rows() {
        for item in grid.get_row(r) {
            if item == &'O' {
                sum += grid.rows() - r;
            }
        }
    }
    println!("{sum}");
}
