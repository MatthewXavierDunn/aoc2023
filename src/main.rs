use std::{
    env::args,
    io::{stdin, BufRead, BufReader},
};

mod trebuchet;
mod cube_conundrum;

fn main() -> std::io::Result<()> {
    let mut args = args().skip(1);
    let n = args.next().expect("Expected AoC identifier, e.g. 1-2");
    let stdin = stdin().lock();
    let lines = BufReader::new(stdin).lines();
    match n.as_str() {
        "1-1" | "trebuchet-part1" => trebuchet::part1(lines),
        "1-2" | "trebuchet-part2" => trebuchet::part2(lines),
        "2-1" | "cube-conundrum-part1" => cube_conundrum::part1(lines),
        "2-2" | "cube-conundrum-part2" => cube_conundrum::part2(lines),
        _ => eprintln!("Unexpected AoC identifier '{n}'"),
    };
    Ok(())
}
