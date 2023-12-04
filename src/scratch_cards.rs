fn wins(lines: impl Iterator<Item = String>) -> impl Iterator<Item = u32> {
    lines.map(|line| {
        let (_, line) = line.split_once(": ").unwrap();
        let (available, winning) = line.split_once(" | ").unwrap();
        let available = available
            .split_whitespace()
            .map(|num_string| num_string.parse::<u32>().unwrap())
            .fold(0u128, |nums, n| nums | (1 << n));
        let winning = winning
            .split_whitespace()
            .map(|num_string| num_string.parse::<u32>().unwrap())
            .fold(0u128, |nums, n| nums | (1 << n));
        (available & winning).count_ones()
    })
}

pub fn part1(lines: impl Iterator<Item = String>) {
    let sum = wins(lines).fold(0u32, |sum, wins| {
        if wins != 0 {
            return sum + (1 << (wins - 1));
        }
        sum
    });
    println!("{sum}");
}

pub fn part2(lines: impl Iterator<Item = String>) {
    let mut multipliers = [1; 11];
    let sum = wins(lines).fold(0, |mut sum, wins| {
        sum += multipliers[0];
        for n in 0..wins {
            multipliers[n as usize + 1] += multipliers[0];
        }
        multipliers.copy_within(1.., 0);
        *multipliers.last_mut().unwrap() = 1;
        sum
    });
    println!("{sum}");
}
