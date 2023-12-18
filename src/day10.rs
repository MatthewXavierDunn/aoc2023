#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
    Ground,
    Animal,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '|' => Self::NS,
            '-' => Self::EW,
            'L' => Self::NE,
            'J' => Self::NW,
            'F' => Self::SE,
            '7' => Self::SW,
            '.' => Self::Ground,
            'S' => Self::Animal,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Dir {
    None,
    Up,
    Down,
    Left,
    Right,
}

fn get(row: usize, col: usize, grid: &Vec<Vec<Tile>>) -> Tile {
    return *grid
        .get(row)
        .and_then(|r| r.get(col))
        .unwrap_or(&Tile::Ground);
}

fn get_next_dir(row: usize, col: usize, dir: Dir, grid: &Vec<Vec<Tile>>) -> Dir {
    match dir {
        Dir::Up => match get(row - 1, col, &grid) {
            Tile::NS => Dir::Up,
            Tile::SE => Dir::Right,
            Tile::SW => Dir::Left,
            _ => Dir::None,
        },
        Dir::Right => match get(row, col + 1, &grid) {
            Tile::EW => Dir::Right,
            Tile::NW => Dir::Up,
            Tile::SW => Dir::Down,
            _ => Dir::None,
        },
        Dir::Down => match get(row + 1, col, &grid) {
            Tile::NS => Dir::Down,
            Tile::NE => Dir::Right,
            Tile::NW => Dir::Left,
            _ => Dir::None,
        },
        Dir::Left => match get(row, col - 1, &grid) {
            Tile::EW => Dir::Left,
            Tile::NE => Dir::Up,
            Tile::SE => Dir::Down,
            _ => Dir::None,
        },
        Dir::None => {
            match get(row - 1, col, &grid) {
                Tile::NS => return Dir::Up,
                Tile::SE => return Dir::Up,
                Tile::SW => return Dir::Up,
                _ => (),
            }
            match get(row, col + 1, &grid) {
                Tile::EW => return Dir::Right,
                Tile::NW => return Dir::Right,
                Tile::SW => return Dir::Right,
                _ => (),
            }
            match get(row + 1, col, &grid) {
                Tile::NS => return Dir::Down,
                Tile::NE => return Dir::Down,
                Tile::NW => return Dir::Down,
                _ => (),
            }
            match get(row, col - 1, &grid) {
                Tile::EW => return Dir::Left,
                Tile::NE => return Dir::Left,
                Tile::SE => return Dir::Left,
                _ => (),
            }
            Dir::None
        }
    }
}

pub fn part1(lines: impl Iterator<Item = String>) {
    let grid = lines
        .map(|line| line.chars().map(|c| Tile::from(c)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let animal_pos = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter().enumerate().find_map(|(c, tile)| {
                if tile == &Tile::Animal {
                    return Some((r, c));
                }
                None
            })
        })
        .unwrap();
    let (mut row, mut col) = animal_pos;
    let mut steps = 0;
    let mut dir = get_next_dir(row, col, Dir::None, &grid);

    loop {
        let new_dir = get_next_dir(row, col, dir, &grid);
        steps += 1;
        match dir {
            Dir::Up => row -= 1,
            Dir::Right => col += 1,
            Dir::Down => row += 1,
            Dir::Left => col -= 1,
            Dir::None => break,
        }
        dir = new_dir;
    }
    println!("{}", steps / 2);
}

pub fn part2(lines: impl Iterator<Item = String>) {
    let grid = lines
        .map(|line| line.chars().map(|c| Tile::from(c)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let animal_pos = grid
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter().enumerate().find_map(|(c, tile)| {
                if tile == &Tile::Animal {
                    return Some((r, c));
                }
                None
            })
        })
        .unwrap();
    let (mut row, mut col) = animal_pos;
    let mut dir = get_next_dir(row, col, Dir::None, &grid);
    let mut polygon = Vec::new();

    loop {
        let new_dir = get_next_dir(row, col, dir, &grid);
        polygon.push((row as i32, col as i32));
        match dir {
            Dir::Up => row -= 1,
            Dir::Right => col += 1,
            Dir::Down => row += 1,
            Dir::Left => col -= 1,
            Dir::None => break,
        }
        dir = new_dir;
    }
    polygon.pop();

    let mut sum = 0;

    for i in 0..polygon.len() - 1 {
        sum += polygon[i].0 * polygon[i + 1].1 - polygon[i + 1].0 * polygon[i].1;
    }
    sum +=
        polygon[polygon.len() - 1].0 * polygon[0].1 - polygon[0].0 * polygon[polygon.len() - 1].1;

    let area = (sum.abs() - polygon.len() as i32 + 3) / 2;
    println!("{area}");
}
