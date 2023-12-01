use std::io::{BufReader, stdin, BufRead};

mod trebuchet;

fn get_calibration(string: &str) -> Option<u32> {
    let first_digit = string.chars().find(|c| c.is_numeric())?.to_digit(10).unwrap();
    let last_digit = string.chars().rev().find(|c| c.is_numeric())?.to_digit(10).unwrap();
    Some(first_digit * 10 + last_digit)
}

fn main() -> std::io::Result<()> {
    let mut sum = 0;
    for line in BufReader::new(stdin()).lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        if let Some(calibration) = get_calibration(&line) {
            sum += calibration;
        }
    }
    println!("{sum}");
    Ok(())
}
