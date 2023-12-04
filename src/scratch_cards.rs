pub fn part1(mut lines: impl Iterator<Item = std::io::Result<String>>) {
    let mut sum = 0;
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (_, line) = line.split_once(": ").unwrap();
        let (available, winning) = line.split_once(" | ").unwrap();
        let available = available
            .split_whitespace()
            .map(|num_string| num_string.parse::<u32>().unwrap())
            .fold(0u128, |nums, n| {
                nums | (1 << n)
            });
        let winning = winning
            .split_whitespace()
            .map(|num_string| num_string.parse::<u32>().unwrap())
            .fold(0u128, |nums, n| {
                nums | (1 << n)
            });
        let wins = (available & winning).count_ones();
        if wins != 0 {
            sum += 1 << (wins - 1);
        }
    }
    println!("{sum}");
}

pub fn part2(mut lines: impl Iterator<Item = std::io::Result<String>>) {
    let mut multipliers = [1; 201];
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (card, line) = line.split_once(": ").unwrap();
        let card_number = card.strip_prefix("Card").unwrap().trim_start().parse::<usize>().unwrap() - 1;
        let (available, winning) = line.split_once(" | ").unwrap();
        let available = available
            .split_whitespace()
            .map(|num_string| num_string.parse::<u32>().unwrap())
            .fold(0u128, |nums, n| {
                nums | (1 << n)
            });
        let winning = winning
            .split_whitespace()
            .map(|num_string| num_string.parse::<u32>().unwrap())
            .fold(0u128, |nums, n| {
                nums | (1 << n)
            });
        let wins = (available & winning).count_ones();
        for n in 0..wins {
            multipliers[card_number + n as usize + 1] += multipliers[card_number];
        }
    }
    let sum: u32 = multipliers.into_iter().sum();
    println!("{sum}");
}
