const ROW_LENGTH: usize = 140;

struct Neighbours {
    tl: char,
    tm: char,
    tr: char,
    cl: char,
    cr: char,
    bl: char,
    bm: char,
    br: char,
}

impl IntoIterator for Neighbours {
    type Item = char;

    type IntoIter = NeighbourIterator;

    fn into_iter(self) -> Self::IntoIter {
        NeighbourIterator {
            neighbours: self,
            n: 0,
        }
    }
}

struct NeighbourIterator {
    neighbours: Neighbours,
    n: u8,
}

impl Iterator for NeighbourIterator {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let res = match self.n {
            0 => Some(self.neighbours.tl),
            1 => Some(self.neighbours.tm),
            2 => Some(self.neighbours.tr),
            3 => Some(self.neighbours.cl),
            4 => Some(self.neighbours.cr),
            5 => Some(self.neighbours.bl),
            6 => Some(self.neighbours.bm),
            7 => Some(self.neighbours.br),
            _ => None,
        };
        self.n += 1;
        res
    }
}

fn get_neighbours(r: usize, c: usize, grid: &Vec<Vec<char>>) -> Neighbours {
    Neighbours {
        tl: *grid
            .get(r - 1)
            .and_then(|row| row.get(c - 1))
            .unwrap_or(&'.'),
        tm: *grid.get(r - 1).and_then(|row| row.get(c)).unwrap_or(&'.'),
        tr: *grid
            .get(r - 1)
            .and_then(|row| row.get(c + 1))
            .unwrap_or(&'.'),
        cl: *grid.get(r).and_then(|row| row.get(c - 1)).unwrap_or(&'.'),
        cr: *grid.get(r).and_then(|row| row.get(c + 1)).unwrap_or(&'.'),
        bl: *grid
            .get(r + 1)
            .and_then(|row| row.get(c - 1))
            .unwrap_or(&'.'),
        bm: *grid.get(r + 1).and_then(|row| row.get(c)).unwrap_or(&'.'),
        br: *grid
            .get(r + 1)
            .and_then(|row| row.get(c + 1))
            .unwrap_or(&'.'),
    }
}

fn prepare_grid(mut lines: impl Iterator<Item = std::io::Result<String>>) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut row = Vec::with_capacity(ROW_LENGTH);
        row.extend(line.chars());
        grid.push(row);
    }
    grid
}

pub fn part1(lines: impl Iterator<Item = std::io::Result<String>>) {
    let grid = prepare_grid(lines);
    let mut sum = 0;
    for (r, row) in grid.iter().enumerate() {
        let mut num = 0;
        let mut valid = false;
        for (c, col) in row.iter().enumerate() {
            if col.is_ascii_digit() {
                num = num * 10 + col.to_digit(10).unwrap();
                for neighbour in get_neighbours(r, c, &grid) {
                    if !neighbour.is_ascii_digit() && neighbour != '.' {
                        valid = true;
                        break;
                    }
                }
            } else {
                if valid {
                    sum += num;
                }
                num = 0;
                valid = false;
            }
        }
    }
    println!("{sum}");
}

fn get_number(r: usize, c: usize, grid: &Vec<Vec<char>>) -> u32 {
    let row = grid.get(r).unwrap();
    let mut left = c;
    let mut right = c;
    loop {
        let mut valid_count = 0;
        if left != 0 {
            if let Some(lchr) = row.get(left - 1) {
                if lchr.is_ascii_digit() {
                    left -= 1;
                    valid_count += 1;
                }
            }
        }
        if let Some(rchr) = row.get(right + 1) {
            if rchr.is_ascii_digit() {
                right += 1;
                valid_count += 1;
            }
        }
        if valid_count == 0 {
            break;
        }
    }
    return row
        .get(left..=right)
        .unwrap()
        .into_iter()
        .collect::<String>()
        .parse()
        .unwrap();
}

pub fn part2(lines: impl Iterator<Item = std::io::Result<String>>) {
    let grid = prepare_grid(lines);
    let mut sum = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, &chr) in row.iter().enumerate() {
            if chr != '*' {
                continue;
            }
            let mut adjacent_nums = 0;
            let mut adjacent_product = 1;
            let neighbours = get_neighbours(r, c, &grid);
            if neighbours.tl.is_ascii_digit() {
                if !neighbours.tm.is_ascii_digit() && neighbours.tr.is_ascii_digit() {
                    adjacent_nums += 1;
                    adjacent_product *= get_number(r - 1, c + 1, &grid);
                }
                adjacent_nums += 1;
                adjacent_product *= get_number(r - 1, c - 1, &grid);
            } else if neighbours.tm.is_ascii_digit() {
                adjacent_nums += 1;
                adjacent_product *= get_number(r - 1, c, &grid);
            } else if neighbours.tr.is_ascii_digit() {
                adjacent_nums += 1;
                adjacent_product *= get_number(r - 1, c + 1, &grid);
            }
            if neighbours.cl.is_ascii_digit() {
                adjacent_nums += 1;
                adjacent_product *= get_number(r, c - 1, &grid);
            }
            if neighbours.cr.is_ascii_digit() {
                adjacent_nums += 1;
                adjacent_product *= get_number(r, c + 1, &grid);
            }
            if neighbours.bl.is_ascii_digit() {
                if !neighbours.bm.is_ascii_digit() && neighbours.br.is_ascii_digit() {
                    adjacent_nums += 1;
                    adjacent_product *= get_number(r + 1, c + 1, &grid);
                }
                adjacent_nums += 1;
                adjacent_product *= get_number(r + 1, c - 1, &grid);
            } else if neighbours.bm.is_ascii_digit() {
                adjacent_nums += 1;
                adjacent_product *= get_number(r + 1, c, &grid);
            } else if neighbours.br.is_ascii_digit() {
                adjacent_nums += 1;
                adjacent_product *= get_number(r + 1, c + 1, &grid);
            }
            if adjacent_nums == 2 {
                sum += adjacent_product;
            }
        }
    }
    println!("{sum}");
}
