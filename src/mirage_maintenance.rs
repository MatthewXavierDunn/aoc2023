pub fn part1(lines: impl Iterator<Item = String>) {
    let x: i32 = lines.map(|line| {
        let mut sequence = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut lasts = Vec::new();
        let mut new_sequence = Vec::with_capacity(sequence.len());
        lasts.push(*sequence.last().unwrap());
        loop {
            let mut all_zero = true;
            new_sequence.extend(sequence.iter().zip(sequence.iter().skip(1)).map(|(a, b)| {
                let c = b - a;
                if c != 0 {
                    all_zero = false;
                }
                c
            }));
            if all_zero {
                break;
            }
            lasts.push(*new_sequence.last().unwrap());
            sequence.clear();
            sequence.extend(new_sequence.drain(..));
        }

        lasts.into_iter().rev().fold(0, |diff, elem| {
            elem + diff
        })
    }).sum();
    println!("{x}");
}

pub fn part2(lines: impl Iterator<Item = String>) {
    let x: i32 = lines.map(|line| {
        let mut sequence = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut firsts = Vec::new();
        let mut new_sequence = Vec::with_capacity(sequence.len());
        firsts.push(*sequence.first().unwrap());
        loop {
            let mut all_zero = true;
            new_sequence.extend(sequence.iter().zip(sequence.iter().skip(1)).map(|(a, b)| {
                let c = b - a;
                if c != 0 {
                    all_zero = false;
                }
                c
            }));
            if all_zero {
                break;
            }
            firsts.push(*new_sequence.first().unwrap());
            sequence.clear();
            sequence.extend(new_sequence.drain(..));
        }

        firsts.into_iter().rev().fold(0, |diff, elem| {
            elem - diff
        })
    }).sum();
    println!("{x}");
}
