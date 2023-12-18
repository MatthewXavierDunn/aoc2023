use crate::grid::Grid;

#[derive(Debug)]
enum Sym {
    Row(usize),
    Col(usize),
}

impl Sym {
    fn get_value(self) -> usize {
        match self {
            Self::Row(row) => row * 100,
            Self::Col(col) => col,
        }
    }
}

fn find_symmetry(grid: &Grid<char>) -> Sym {
    let mut found_col_sym = None;
    for c in 1..grid.cols() {
        let mut sym = true;
        for i in 0..grid.cols().abs_diff(c).min(c) {
            if !grid.get_col(c - i - 1).eq(grid.get_col(c + i)) {
                sym = false;
                break;
            }
        }
        if sym {
            found_col_sym = Some(c);
            break;
        }
    }
    if let Some(sym) = found_col_sym {
        return Sym::Col(sym);
    }
    let mut found_row_sym = None;
    for r in 1..grid.rows() {
        let mut sym = true;
        for i in 0..grid.rows().abs_diff(r).min(r) {
            if !grid.get_row(r - i - 1).eq(grid.get_row(r + i)) {
                sym = false;
                break;
            }
        }
        if sym {
            found_row_sym = Some(r);
            break;
        }
    }
    if let Some(sym) = found_row_sym {
        return Sym::Row(sym);
    }
    unreachable!()
}

fn make_symmetry(grid: &Grid<char>) -> Sym {
    let mut found_row_sym = None;
    for r in 1..grid.rows() {
        let mut sym = true;
        let mut changed = false;
        for i in 0..grid.rows().abs_diff(r).min(r) {
            let mut top = grid.get_row(r - i - 1);
            let mut bottom = grid.get_row(r + i);
            loop {
                let (Some(top), Some(bottom)) = (top.next(), bottom.next()) else {
                    break;
                };
                if top != bottom {
                    if !changed {
                        changed = true;
                    } else {
                        sym = false;
                        break;
                    }
                }
            }
            if !sym {
                break;
            }
        }
        if sym && changed {
            found_row_sym = Some(r);
            break;
        }
    }
    if let Some(sym) = found_row_sym {
        return Sym::Row(sym);
    }
    let mut found_col_sym = None;
    for c in 1..grid.cols() {
        let mut sym = true;
        let mut changed = false;
        for i in 0..grid.cols().abs_diff(c).min(c) {
            let mut left = grid.get_col(c - i - 1);
            let mut right = grid.get_col(c + i);
            loop {
                let (Some(left), Some(right)) = (left.next(), right.next()) else {
                    break;
                };
                if left != right {
                    if !changed {
                        changed = true;
                    } else {
                        sym = false;
                        break;
                    }
                }
            }
            if !sym {
                break;
            }
        }
        if sym && changed {
            found_col_sym = Some(c);
            break;
        }
    }
    if let Some(sym) = found_col_sym {
        return Sym::Col(sym);
    }
    unreachable!()
}

pub fn part1(mut lines: impl Iterator<Item = String>) {
    let mut grid = Grid::new();
    let mut sum = 0;
    loop {
        let line = lines.next();
        if let Some(line) = line {
            if line.is_empty() {
                sum += find_symmetry(&grid).get_value();
                grid.clear();
            } else {
                grid.push_row(line.chars());
            }
        } else {
            sum += find_symmetry(&grid).get_value();
            break;
        }
    }
    println!("{sum}");
}

pub fn part2(mut lines: impl Iterator<Item = String>) {
    let mut grid = Grid::new();
    let mut sum = 0;
    loop {
        let line = lines.next();
        if let Some(line) = line {
            if line.is_empty() {
                sum += make_symmetry(&grid).get_value();
                grid.clear();
            } else {
                grid.push_row(line.chars());
            }
        } else {
            sum += make_symmetry(&grid).get_value();
            break;
        }
    }
    println!("{sum}");
}
