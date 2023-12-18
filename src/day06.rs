pub fn part1(mut lines: impl Iterator<Item = String>) {
    let times_line = lines.next().unwrap();
    let times = times_line
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(|t| t.parse::<u64>().unwrap());
    let distances_line = lines.next().unwrap();
    let distances = distances_line
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(|t| t.parse::<u64>().unwrap());

    let prod = times
        .zip(distances)
        .map(|(time, distance)| {
            let min_x = ((-(time as f64) + f64::sqrt((time * time - 4 * distance) as f64)) * -0.5
                + 1.0)
                .floor() as u64;
            let max_x = ((-(time as f64) - f64::sqrt((time * time - 4 * distance) as f64)) * -0.5
                - 1.0)
                .ceil() as u64;
            max_x - min_x + 1
        })
        .product::<u64>();

    println!("{prod}");
}

pub fn part2(mut lines: impl Iterator<Item = String>) {
    let time_line = lines.next().unwrap();
    let time: u128 = time_line
        .strip_prefix("Time:")
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();
    let distance_line = lines.next().unwrap();
    let distance: u128 = distance_line
        .strip_prefix("Distance:")
        .unwrap()
        .replace(" ", "")
        .parse()
        .unwrap();

    let min_x = ((-(time as f64) + f64::sqrt((time * time - 4 * distance) as f64)) * -0.5 + 1.0)
        .floor() as u64;
    let max_x = ((-(time as f64) - f64::sqrt((time * time - 4 * distance) as f64)) * -0.5 - 1.0)
        .ceil() as u64;
    let res = max_x - min_x + 1;

    println!("{res}");
}
