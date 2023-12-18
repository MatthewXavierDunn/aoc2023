use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

fn arrangements(
    cache: &mut HashMap<(bool, Vec<Spring>, Vec<usize>), usize>,
    seen_damaged: bool,
    springs: &mut [Spring],
    records: &mut [usize],
) -> usize {
    if let Some(n) = cache.get(&(seen_damaged, springs.to_vec(), records.to_vec())) {
        return *n;
    }
    if springs.len() == 0 && (records.len() == 0 || (records.len() == 1 && records[0] == 0)) {
        cache.insert((seen_damaged, springs.to_vec(), records.to_vec()), 1);
        return 1;
    }
    if springs.len() == 0 && records.len() != 0 {
        cache.insert((seen_damaged, springs.to_vec(), records.to_vec()), 0);
        return 0;
    }
    match springs[0] {
        Spring::Operational => {
            if seen_damaged && records.len() != 0 && records[0] == 0 {
                let n = arrangements(cache, false, &mut springs[1..], &mut records[1..]);
                cache.insert((seen_damaged, springs.to_vec(), records.to_vec()), n);
                return n;
            }
            if !seen_damaged {
                let n = arrangements(cache, seen_damaged, &mut springs[1..], records);
                cache.insert((seen_damaged, springs.to_vec(), records.to_vec()), n);
                return n;
            }
            cache.insert((seen_damaged, springs.to_vec(), records.to_vec()), 0);
            return 0;
        }
        Spring::Damaged => {
            if records.len() == 0 {
                cache.insert((seen_damaged, springs.to_vec(), records.to_vec()), 0);
                return 0;
            }
            if records[0] == 0 {
                cache.insert((seen_damaged, springs.to_vec(), records.to_vec()), 0);
                return 0;
            }
            records[0] -= 1;
            let count = arrangements(cache, true, &mut springs[1..], records);
            records[0] += 1;
            cache.insert((seen_damaged, springs.to_vec(), records.to_vec()), count);
            return count;
        }
        Spring::Unknown => {
            springs[0] = Spring::Operational;
            let count1 = arrangements(cache, seen_damaged, springs, records);

            springs[0] = Spring::Damaged;
            let count2 = arrangements(cache, seen_damaged, springs, records);

            springs[0] = Spring::Unknown;

            cache.insert(
                (seen_damaged, springs.to_vec(), records.to_vec()),
                count1 + count2,
            );

            return count1 + count2;
        }
    }
}

pub fn part1(lines: impl Iterator<Item = String>) {
    let sum = lines
        .map(|line| {
            let mut parts = line.split_whitespace();
            let mut springs = parts
                .next()
                .unwrap()
                .chars()
                .map(|c| match c {
                    '.' => Spring::Operational,
                    '#' => Spring::Damaged,
                    '?' => Spring::Unknown,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();
            let mut records = parts
                .next()
                .unwrap()
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            arrangements(&mut HashMap::new(), false, &mut springs, &mut records)
        })
        .sum::<usize>();
    println!("{sum}");
}

pub fn part2(lines: impl Iterator<Item = String>) {
    let sum = lines
        .map(|line| {
            let mut parts = line.split_whitespace();
            let springs = parts
                .next()
                .unwrap()
                .chars()
                .map(|c| match c {
                    '.' => Spring::Operational,
                    '#' => Spring::Damaged,
                    '?' => Spring::Unknown,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();
            let mut springs_run = Vec::new();
            springs_run.extend_from_slice(&springs[..]);
            springs_run.push(Spring::Unknown);
            springs_run.extend_from_slice(&springs[..]);
            springs_run.push(Spring::Unknown);
            springs_run.extend_from_slice(&springs[..]);
            springs_run.push(Spring::Unknown);
            springs_run.extend_from_slice(&springs[..]);
            springs_run.push(Spring::Unknown);
            springs_run.extend_from_slice(&springs[..]);

            let records = parts
                .next()
                .unwrap()
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let mut records_run = Vec::new();
            records_run.extend_from_slice(&records[..]);
            records_run.extend_from_slice(&records[..]);
            records_run.extend_from_slice(&records[..]);
            records_run.extend_from_slice(&records[..]);
            records_run.extend_from_slice(&records[..]);

            arrangements(
                &mut HashMap::new(),
                false,
                &mut springs_run,
                &mut records_run,
            )
        })
        .sum::<usize>();
    println!("{sum}");
}
