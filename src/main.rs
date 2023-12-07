use std::{
    env::args,
    io::{stdin, BufRead, BufReader},
};

mod cube_conundrum;
mod gear_ratios;
mod scratch_cards;
mod seed_fertilizer;
mod trebuchet;
mod wait_for_it;

fn main() -> std::io::Result<()> {
    let mut args = args().skip(1);
    let n = args.next().expect("Expected AoC identifier, e.g. 1-2");
    let stdin = stdin().lock();
    let lines = BufReader::new(stdin).lines().flatten();
    match n.as_str() {
        "1-1" | "trebuchet-part1" => trebuchet::part1(lines),
        "1-2" | "trebuchet-part2" => trebuchet::part2(lines),
        "2-1" | "cube-conundrum-part1" => cube_conundrum::part1(lines),
        "2-2" | "cube-conundrum-part2" => cube_conundrum::part2(lines),
        "3-1" | "gear-ratios-part1" => gear_ratios::part1(lines),
        "3-2" | "gear-ratios-part2" => gear_ratios::part2(lines),
        "4-1" | "scratch-cards-part1" => scratch_cards::part1(lines),
        "4-2" | "scratch-cards-part2" => scratch_cards::part2(lines),
        "5-1" | "seed-fertilizer-part1" => seed_fertilizer::part1(lines),
        "5-2" | "seed-fertilizer-part2" => seed_fertilizer::part2(lines),
        "6-1" | "wait-for-it-part1" => wait_for_it::part1(lines),
        "6-2" | "wait-for-it-part2" => wait_for_it::part2(lines),
        _ => eprintln!("Unexpected AoC identifier '{n}'"),
    };
    Ok(())
}
