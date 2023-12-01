use std::{
    env::args,
    io::{stdin, BufRead, BufReader, ErrorKind},
};

mod aoc;
mod trebuchet;

use aoc::Aoc;
use trebuchet::Trebuchet;

fn main() -> std::io::Result<()> {
    let mut args = args();
    let _ = args.next();
    let n = args.next().ok_or(ErrorKind::NotFound)?;
    let stdin = stdin().lock();
    let lines = BufReader::new(stdin).lines();
    match n.as_str() {
        "1" => Trebuchet::solve(lines),
        _ => Err(ErrorKind::Unsupported.into()),
    }
}
