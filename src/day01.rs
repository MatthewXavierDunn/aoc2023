use std::num::NonZeroU8;

const NUMERICS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_non_zero_u8(c: char) -> Option<NonZeroU8> {
    if c.is_ascii_digit() && c != '0' {
        return Some(unsafe {
            // Safety: since we have found a non-zero numeric character, conversion to a non-zero
            // digit will always be safe.
            NonZeroU8::new_unchecked(c.to_digit(10).unwrap_unchecked() as u8)
        });
    }
    None
}

fn parse_tigid(string: &str) -> Option<NonZeroU8> {
    parse_non_zero_u8(string.chars().rev().next()?)
}

fn parse_digit(string: &str) -> Option<NonZeroU8> {
    parse_non_zero_u8(string.chars().next()?)
}

fn parse_numeric(string: &str) -> Option<NonZeroU8> {
    NUMERICS
        .into_iter()
        .enumerate()
        .find_map(|(i, numeric)| {
            if string.starts_with(numeric) {
                Some(unsafe {
                    // Safety: enumeration starts at 0, thus the minimal value will be 1.
                    NonZeroU8::new_unchecked(i as u8 + 1)
                })
            } else {
                None
            }
        })
        .or_else(|| parse_digit(string))
}

fn parse_ciremun(string: &str) -> Option<NonZeroU8> {
    NUMERICS
        .into_iter()
        .enumerate()
        .find_map(|(i, numeric)| {
            if string.ends_with(numeric) {
                Some(unsafe {
                    // Safety: enumeration starts at 0, thus the minimal value will be 1.
                    NonZeroU8::new_unchecked(i as u8 + 1)
                })
            } else {
                None
            }
        })
        .or_else(|| parse_tigid(string))
}

fn get_calibration(
    mut lines: impl Iterator<Item = String>,
    forwards: impl Fn(&str) -> Option<NonZeroU8>,
    backwards: impl Fn(&str) -> Option<NonZeroU8>,
) -> u64 {
    let mut calibration = 0;
    while let Some(line) = lines.next() {
        let mut line = line.as_str();
        let mut first = None;
        while !line.is_empty() {
            if let Some(n) = forwards(line) {
                first = Some(n);
                break;
            }
            // Safety: since the line is not empty, we know that it is possible to still get a
            // slice from the first index.
            line = unsafe { line.get(1..).unwrap_unchecked() };
        }

        let mut last = first;
        while !line.is_empty() {
            if let Some(n) = backwards(line) {
                last = Some(n);
                break;
            }
            // Safety: since the line is not empty, we know that it is possible to still get a
            // slice up to the penultimate index.
            line = unsafe { line.get(..line.len() - 1).unwrap_unchecked() };
        }

        if let (Some(first), Some(last)) = (first, last) {
            calibration += (first.get() * 10 + last.get()) as u64;
        }
    }
    calibration
}

pub fn part1(lines: impl Iterator<Item = String>) {
    let calibration = get_calibration(lines, parse_digit, parse_tigid);
    println!("{calibration}");
}

pub fn part2(lines: impl Iterator<Item = String>) {
    let calibration = get_calibration(lines, parse_numeric, parse_ciremun);
    println!("{calibration}");
}
