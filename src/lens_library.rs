pub fn part1(mut lines: impl Iterator<Item = String>) {
    let mut sum = 0;
    for instruction in lines.next().unwrap().split(',') {
        sum += hash(instruction);
    }
    println!("{sum}");
}

pub fn part2(mut lines: impl Iterator<Item = String>) {
    let mut boxes = Vec::with_capacity(256);
    for _ in 0..256 {
        boxes.push(Vec::new());
    }
    let line = lines.next().unwrap();
    for instruction in line.split(',') {
        if instruction.ends_with('-') {
            let label = &instruction[..instruction.len() - 1];
            let box_num = hash(label);
            boxes[box_num as usize].retain(|(lbl, _)| lbl != &label);
        } else {
            let (label, focal_len) = instruction.split_once('=').unwrap();
            let box_num = hash(label);
            let focal_len: u8 = focal_len.parse().unwrap();
            if boxes[box_num as usize]
                .iter_mut()
                .find(|(lbl, _)| lbl == &label)
                .and_then(|(_, lens)| {
                    *lens = focal_len;
                    Some(())
                })
                .is_none()
            {
                boxes[box_num as usize].push((label, focal_len));
            };
        }
    }
    let mut sum: usize = 0;
    for (i, b) in boxes.into_iter().enumerate() {
        sum += b
            .into_iter()
            .enumerate()
            .map(|(j, (_, lens))| (i + 1) * (j + 1) * lens as usize)
            .sum::<usize>();
    }
    println!("{sum}");
}

fn hash(string: &str) -> u8 {
    let mut val: u8 = 0;
    for char in string.chars() {
        val = val.wrapping_add(char as u8);
        val = val.wrapping_mul(17);
    }
    val
}
