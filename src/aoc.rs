use std::io::{BufReader, Lines, StdinLock};

pub trait Aoc {
    fn solve(lines: Lines<BufReader<StdinLock<'_>>>) -> std::io::Result<()>;
}
