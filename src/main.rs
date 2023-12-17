use std::{
    env::args,
    io::{stdin, BufRead, BufReader},
};

mod camel_cards;
mod cosmic_expansion;
mod cube_conundrum;
mod gear_ratios;
mod haunted_wasteland;
mod mirage_maintenance;
mod pipe_maze;
mod scratch_cards;
mod seed_fertilizer;
mod trebuchet;
mod wait_for_it;

fn main() -> std::io::Result<()> {
    let mut args = args().skip(1);
    let n = args
        .next()
        .expect("Expected AoC identifier, e.g. 1-2 for day 1 part 2");
    let stdin = stdin().lock();
    let lines = BufReader::new(stdin).lines().flatten();
    match n.as_str() {
        "1-1" => trebuchet::part1(lines),
        "1-2" => trebuchet::part2(lines),
        "2-1" => cube_conundrum::part1(lines),
        "2-2" => cube_conundrum::part2(lines),
        "3-1" => gear_ratios::part1(lines),
        "3-2" => gear_ratios::part2(lines),
        "4-1" => scratch_cards::part1(lines),
        "4-2" => scratch_cards::part2(lines),
        "5-1" => seed_fertilizer::part1(lines),
        "5-2" => seed_fertilizer::part2(lines),
        "6-1" => wait_for_it::part1(lines),
        "6-2" => wait_for_it::part2(lines),
        "7-1" => camel_cards::part1(lines),
        "7-2" => camel_cards::part2(lines),
        "8-1" => haunted_wasteland::part1(lines),
        "8-2" => haunted_wasteland::part2(lines),
        "9-1" => mirage_maintenance::part1(lines),
        "9-2" => mirage_maintenance::part2(lines),
        "10-1" => pipe_maze::part1(lines),
        "10-2" => pipe_maze::part2(lines),
        "11-1" => cosmic_expansion::part1(lines),
        "11-2" => cosmic_expansion::part2(lines),
        _ => eprintln!("Unexpected AoC identifier '{n}'"),
    };
    Ok(())
}
