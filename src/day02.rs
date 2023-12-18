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

fn parse_games(lines: impl Iterator<Item = String>) -> impl Iterator<Item = Game> {
    lines.map(|line| parse_game(&line))
}

pub fn part1(lines: impl Iterator<Item = String>) {
    let game_id_sum = parse_games(lines).fold(0, |sum, game| {
        sum + game
            .sets
            .iter()
            .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14)
            .then_some(game.game_id)
            .unwrap_or_default()
    });
    println!("{game_id_sum}");
}

pub fn part2(lines: impl Iterator<Item = String>) {
    let power_sum = parse_games(lines).fold(0, |sum, game| {
        sum + game
            .sets
            .iter()
            .fold([0, 0, 0], |[red, green, blue], set| {
                [red.max(set.red), green.max(set.green), blue.max(set.blue)]
            })
            .iter()
            .product::<u32>()
    });
    println!("{power_sum}");
}
