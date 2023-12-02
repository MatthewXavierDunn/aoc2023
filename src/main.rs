use std::{
    env::args,
    io::{stdin, BufRead, BufReader, ErrorKind},
};

mod trebuchet;

fn main() -> std::io::Result<()> {
    let mut args = args();
    let _ = args.next();
    let n = args.next().ok_or(ErrorKind::NotFound)?;
    let stdin = stdin().lock();
    let lines = BufReader::new(stdin).lines();
    match n.as_str() {
        "1-1" | "trebuchet-part1" => trebuchet::part1(lines),
        "1-2" | "trebuchet-part2" => trebuchet::part2(lines),
        _ => return Err(ErrorKind::Unsupported.into()),
    };
    Ok(())
}
