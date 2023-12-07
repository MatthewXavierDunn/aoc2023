pub fn part1(mut lines: impl Iterator<Item = String>) {
    let seed_line = lines.next().unwrap();
    let seeds = seed_line
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap());

    let mut mappings = Vec::new();
    lines.next().unwrap();

    while lines.next().is_some() {
        let mut index_map = Vec::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let mut map_line = line.split_whitespace();
            let dst_start = map_line.next().unwrap().parse::<u64>().unwrap();
            let src_start = map_line.next().unwrap().parse::<u64>().unwrap();
            let rng = map_line.next().unwrap().parse::<u64>().unwrap();

            index_map.push((src_start..src_start + rng, dst_start));
        }

        mappings.push(index_map);
    }

    let mut min = u64::MAX;
    for seed in seeds {
        let mut current_index = seed;
        for mapping in &mappings {
            for (src_rng, dst_start) in mapping {
                if src_rng.contains(&current_index) {
                    current_index = dst_start + (current_index - src_rng.start);
                    break;
                }
            }
        }
        if current_index < min {
            min = current_index;
        }
    }
    println!("{min}");
}

pub fn part2(mut lines: impl Iterator<Item = String>) {
    let seed_line = lines.next().unwrap();
    let seeds = seed_line
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .fold((None, Vec::new()), |(start, mut seeds), n| match start {
            Some(m) => {
                seeds.push(vec![m..m + n]);
                (None, seeds)
            }
            None => (Some(n), seeds),
        })
        .1;

    let mut mappings = Vec::new();
    lines.next().unwrap();

    while lines.next().is_some() {
        let mut index_map = Vec::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let mut map_line = line.split_whitespace();
            let dst_start = map_line.next().unwrap().parse::<u64>().unwrap();
            let src_start = map_line.next().unwrap().parse::<u64>().unwrap();
            let rng = map_line.next().unwrap().parse::<u64>().unwrap();

            index_map.push((src_start..src_start + rng, dst_start));
        }

        mappings.push(index_map);
    }

    let mut min = u64::MAX;
    for seed in seeds {
        let mut ranges = Vec::from_iter(seed);
        let mut result_ranges = Vec::with_capacity(ranges.len());
        for mapping in &mappings {
            while let Some(rng) = ranges.pop() {
                let mut found_mapping = false;
                for (src_rng, dst_start) in mapping {
                    let offset = *dst_start as i64 - src_rng.start as i64;
                    // full overlap
                    if src_rng.contains(&rng.start) && src_rng.contains(&rng.end) {
                        result_ranges.push(
                            rng.start.checked_add_signed(offset).unwrap()
                                ..rng.end.checked_add_signed(offset).unwrap(),
                        );
                        found_mapping = true;
                        break;
                    }
                    // start overlap
                    if src_rng.contains(&rng.start) {
                        result_ranges.push(
                            rng.start.checked_add_signed(offset).unwrap()
                                ..src_rng.end.checked_add_signed(offset).unwrap(),
                        );
                        ranges.push(src_rng.end..rng.end);
                        found_mapping = true;
                        break;
                    }
                    // end overlap
                    if src_rng.contains(&rng.start) {
                        result_ranges.push(
                            src_rng.start.checked_add_signed(offset).unwrap()
                                ..rng.end.checked_add_signed(offset).unwrap(),
                        );
                        ranges.push(rng.start..src_rng.start);
                        found_mapping = true;
                        break;
                    }
                }
                if !found_mapping {
                    result_ranges.push(rng);
                }
            }
            ranges.extend(result_ranges.drain(..));
        }
        min = min.min(ranges.into_iter().map(|r| r.start).min().unwrap());
    }
    println!("{min}");
}
