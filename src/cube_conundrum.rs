pub fn part1(mut lines: impl Iterator<Item = std::io::Result<String>>) {
    let mut game_id_sum = 0;
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (game_string, line) = line.split_once(": ").unwrap();
        let (_, game_id) = game_string.split_once(" ").unwrap();
        let game_id: u32 = game_id.parse().unwrap();
        let sets = line.split("; ");
        let mut is_valid = true;
        for set in sets {
            let counts = set.split(", ");
            let mut red_total = 0;
            let mut green_total = 0;
            let mut blue_total = 0;
            for count in counts {
                let (n, colour) = count.split_once(' ').unwrap();
                let n: u32 = n.parse().unwrap();
                match colour {
                    "red" => red_total += n,
                    "green" => green_total += n,
                    "blue" => blue_total += n,
                    _ => (),
                }
            }
            if red_total > 12 || green_total > 13 || blue_total > 14 {
                is_valid = false;
            }
        }
        if is_valid {
            game_id_sum += game_id;
        }
    }
    println!("{game_id_sum}");
}

pub fn part2(mut lines: impl Iterator<Item = std::io::Result<String>>) {
    let mut power_sum = 0;
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (_, line) = line.split_once(": ").unwrap();
        let sets = line.split("; ");
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for set in sets {
            let counts = set.split(", ");
            for count in counts {
                let (n, colour) = count.split_once(' ').unwrap();
                let n: u32 = n.parse().unwrap();
                match colour {
                    "red" => if n > max_red {
                        max_red = n;
                    },
                    "green" => if n > max_green {
                        max_green = n;
                    },
                    "blue" => if n > max_blue {
                        max_blue = n;
                    },
                    _ => (),
                }
            }
        }
        power_sum += max_red * max_green * max_blue;
    }
    println!("{power_sum}");
}
