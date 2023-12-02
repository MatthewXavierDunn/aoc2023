struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    game_id: u32,
    sets: Vec<Set>,
}

fn parse_set(string: &str) -> Set {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for count in string.split(", ") {
        let (n, colour) = count.split_once(' ').unwrap();
        let n: u32 = n.parse().unwrap();
        match colour {
            "red" => red += n,
            "green" => green += n,
            "blue" => blue += n,
            _ => (),
        }
    }

    Set { red, green, blue }
}

fn parse_game(string: &str) -> Game {
    let (game_string, line) = string.split_once(": ").unwrap();
    let (_, game_id) = game_string.split_once(" ").unwrap();
    let game_id: u32 = game_id.parse().unwrap();
    let sets = line.split("; ").map(parse_set).collect();

    Game { game_id, sets }
}

fn parse_games(lines: impl Iterator<Item = std::io::Result<String>>) -> impl Iterator<Item = Game> {
    lines.filter_map(|line| {
        line.ok()
            .filter(|line| !line.is_empty())
            .map(|line| parse_game(&line))
    })
}

pub fn part1(lines: impl Iterator<Item = std::io::Result<String>>) {
    let mut game_id_sum = 0;
    for game in parse_games(lines) {
        let mut is_valid = true;
        for set in game.sets {
            if set.red > 12 || set.green > 13 || set.blue > 14 {
                is_valid = false;
                break;
            }
        }
        if is_valid {
            game_id_sum += game.game_id;
        }
    }
    println!("{game_id_sum}");
}

pub fn part2(lines: impl Iterator<Item = std::io::Result<String>>) {
    let mut power_sum = 0;
    for game in parse_games(lines) {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for set in game.sets {
            max_red = max_red.max(set.red);
            max_green = max_green.max(set.green);
            max_blue = max_blue.max(set.blue);
        }
        power_sum += max_red * max_green * max_blue;
    }
    println!("{power_sum}");
}
