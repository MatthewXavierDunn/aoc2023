use core::hash::Hash;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Node {
    inner: [u8; 3],
}

impl From<&str> for Node {
    fn from(str: &str) -> Self {
        let mut chars = str.chars();
        Self {
            inner: [
                chars.next().unwrap() as u8,
                chars.next().unwrap() as u8,
                chars.next().unwrap() as u8,
            ],
        }
    }
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(self.inner[0]);
        state.write_u8(self.inner[1]);
        state.write_u8(self.inner[2]);
    }
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    return a;
}

fn lcm(a: u128, b: u128) -> u128 {
    return (a * b) / gcd(a, b);
}

fn inverse(a: u128, n: u128) -> u128 {
    let mut t = 0;
    let mut r = n;
    let mut new_t = 1;
    let mut new_r = a;

    while new_r != 0 {
        let quotient = r / new_r;
        (t, new_t) = (new_t, t - quotient * new_t);
        (r, new_r) = (new_r, r - quotient * new_r);
    }

    t
}

pub fn part1(mut lines: impl Iterator<Item = String>) {
    let instruction_line = lines.next().unwrap();
    let mut instructions = instruction_line
        .chars()
        .map(|d| if d == 'L' { 0 } else { 1 })
        .cycle();

    lines.next().unwrap();

    let mappings = lines.fold(HashMap::new(), |mut mappings, line| {
        let (node, map) = line.split_once(" = ").unwrap();
        let (map_l, map_r) = map
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_once(", ")
            .unwrap();

        mappings.insert(Node::from(node), [Node::from(map_l), Node::from(map_r)]);
        mappings
    });

    let mut current_node = Node::from("AAA");
    let mut steps = 0;
    let finish_node = Node::from("ZZZ");
    while current_node != finish_node {
        for (node, mapping) in &mappings {
            if &current_node == node {
                current_node = mapping[instructions.next().unwrap()];
                break;
            }
        }
        steps += 1;
    }
    println!("{steps}");
}

pub fn part2(mut lines: impl Iterator<Item = String>) {
    let instruction_line = lines.next().unwrap();
    let instructions = instruction_line
        .chars()
        .map(|d| if d == 'L' { 0 } else { 1 })
        .collect::<Vec<_>>();

    lines.next().unwrap();

    let mappings = lines.fold(HashMap::new(), |mut mappings, line| {
        let (node, map) = line.split_once(" = ").unwrap();
        let (map_l, map_r) = map
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_once(", ")
            .unwrap();

        mappings.insert(Node::from(node), [Node::from(map_l), Node::from(map_r)]);
        mappings
    });

    let mut current_nodes = Vec::new();
    for node in mappings.keys() {
        if node.inner[2] == b'A' {
            current_nodes.push(*node);
        }
    }

    let mut visited = HashMap::new();
    let mut cycles = Vec::new();

    for mut current_node in current_nodes {
        let mut steps = 0u128;
        visited.clear();
        let mut instructions = instructions.iter().enumerate().cycle();
        loop {
            let (instruction_num, instruction) = instructions.next().unwrap();
            if let Some(cycle_start_step) = visited.get(&(instruction_num, current_node)) {
                let cycle_len = steps - cycle_start_step;
                cycles.push(cycle_len);
                break;
            }
            visited.insert((instruction_num, current_node), steps);
            let mappings = mappings.get(&current_node).unwrap();
            current_node = mappings[*instruction];
            steps += 1;
        }
    }

    let steps = cycles.into_iter().fold(1, |lcm_sofar, cycle| {
        lcm(lcm_sofar, cycle)
    });

    println!("{steps}");
}
