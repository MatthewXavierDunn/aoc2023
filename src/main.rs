use std::{
    env::args,
    io::{stdin, BufRead, BufReader},
};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod grid;

fn main() -> std::io::Result<()> {
    let mut args = args().skip(1);
    let n = args
        .next()
        .expect("Expected AoC identifier, e.g. 1-2 for day 1 part 2");
    let stdin = stdin().lock();
    let lines = BufReader::new(stdin).lines().flatten();
    match n.as_str() {
        "1-1" => day01::part1(lines),
        "1-2" => day01::part2(lines),
        "2-1" => day02::part1(lines),
        "2-2" => day02::part2(lines),
        "3-1" => day03::part1(lines),
        "3-2" => day03::part2(lines),
        "4-1" => day04::part1(lines),
        "4-2" => day04::part2(lines),
        "5-1" => day05::part1(lines),
        "5-2" => day05::part2(lines),
        "6-1" => day06::part1(lines),
        "6-2" => day06::part2(lines),
        "7-1" => day07::part1(lines),
        "7-2" => day07::part2(lines),
        "8-1" => day08::part1(lines),
        "8-2" => day08::part2(lines),
        "9-1" => day09::part1(lines),
        "9-2" => day09::part2(lines),
        "10-1" => day10::part1(lines),
        "10-2" => day10::part2(lines),
        "11-1" => day11::part1(lines),
        "11-2" => day11::part2(lines),
        "12-1" => day12::part1(lines),
        "12-2" => day12::part2(lines),
        "13-1" => day13::part1(lines),
        "13-2" => day13::part2(lines),
        "14-1" => day14::part1(lines),
        "14-2" => day14::part2(lines),
        "15-1" => day15::part1(lines),
        "15-2" => day15::part2(lines),
        _ => eprintln!("Unexpected AoC identifier '{n}'"),
    };
    Ok(())
}
