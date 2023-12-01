use std::io::{stdin, BufRead, BufReader};

const NUMERICS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_numeric(string: &str) -> (Option<u8>, usize) {
    for (i, numeric) in NUMERICS.into_iter().enumerate() {
        if string.starts_with(numeric) {
            return (Some(i as u8 + 1), 1);
        }
    }
    if let Some(c) = string.chars().next() {
        if c.is_ascii_digit() {
            // Safety: since we have found a numeric character, conversion to a digit will always
            // be safe.
            unsafe {
                return (Some(c.to_digit(10).unwrap_unchecked() as u8), 1);
            }
        }
    }
    return (None, 1);
}

fn get_calibration(mut string: &str) -> Option<u8> {
    let mut first = None;
    let mut last = None;

    while !string.is_empty() {
        let (res, advance) = parse_numeric(string);
        match res {
            Some(n) => {
                if first.is_none() {
                    first = Some(n);
                }
                last = Some(n);
            }
            None => (),
        }
        string = string.get(advance..).unwrap_or("");
    }

    if let (Some(first), Some(last)) = (first, last) {
        Some(first * 10 + last)
    } else {
        None
    }
}

fn main() -> std::io::Result<()> {
    let mut sum = 0;
    let stdin = stdin().lock();
    let mut lines = BufReader::new(stdin).lines();
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }
        if let Some(calibration) = get_calibration(&line) {
            println!("{calibration} = {line}");
            sum += calibration as u64;
        }
    }
    println!("{sum}");
    Ok(())
}
