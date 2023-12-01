use crate::aoc::Aoc;

const NUMERICS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_numeric(string: &str) -> Option<u8> {
    for (i, numeric) in NUMERICS.into_iter().enumerate() {
        if string.starts_with(numeric) {
            return Some(i as u8 + 1);
        }
    }
    let c = string.chars().next()?;
    if c.is_ascii_digit() {
        Some(unsafe {
            // Safety: since we have found a numeric character, conversion to a digit will always
            // be safe.
            c.to_digit(10).unwrap_unchecked() as u8
        })
    } else {
        None
    }
}

fn get_calibration(mut string: &str) -> Option<u8> {
    let mut first = None;
    let mut last = None;

    while !string.is_empty() {
        let res = parse_numeric(string);
        match res {
            Some(n) => {
                if first.is_none() {
                    first = Some(n);
                }
                last = Some(n);
            }
            None => (),
        }
        string = string.get(1..).unwrap_or("");
    }

    if let (Some(first), Some(last)) = (first, last) {
        Some(first * 10 + last)
    } else {
        None
    }
}

pub struct Trebuchet;

impl Aoc for Trebuchet {
    fn solve(
        mut lines: std::io::Lines<std::io::BufReader<std::io::StdinLock<'_>>>,
    ) -> std::io::Result<()> {
        let mut sum = 0;
        while let Some(Ok(line)) = lines.next() {
            if line.is_empty() {
                break;
            }
            if let Some(calibration) = get_calibration(&line) {
                sum += calibration as u64;
            }
        }
        println!("{sum}");
        Ok(())
    }
}
