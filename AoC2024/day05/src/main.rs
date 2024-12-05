use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let now = std::time::Instant::now();
    let (r, u) = parse();
    let part1 = part1(&r, &u);
    let part2 = part2(&r, u);
    let elapsed = now.elapsed();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Elapsed: {:?}", elapsed);
}

fn part1(restrictions: &HashMap<u16, HashSet<u16>>, updates: &[Vec<u16>]) -> u16 {
    updates
        .iter()
        .filter(|u| u.is_sorted_by(|a, b| restrictions[a].contains(b)))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part2(restrictions: &HashMap<u16, HashSet<u16>>, mut updates: Vec<Vec<u16>>) -> u16 {
    updates
        .iter_mut()
        .filter(|u| !u.is_sorted_by(|a, b| restrictions[a].contains(b)))
        .map(|update| {
            update.sort_by(|a, b| {
                if restrictions[a].contains(b) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            update[update.len() / 2]
        })
        .sum()
}

fn parse() -> (HashMap<u16, HashSet<u16>>, Vec<Vec<u16>>) {
    let (restrictions_str, updates_str) = INPUT.split_once("\r\n\r\n").unwrap();

    let mut restrictions = HashMap::new();
    for line in restrictions_str.lines() {
        let r = line
            .split_once("|")
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .unwrap();

        restrictions
            .entry(r.0)
            .or_insert_with(HashSet::new)
            .insert(r.1);
    }

    let mut updates = Vec::new();
    for update in updates_str.lines() {
        let us = update.split(',').map(|e| e.parse().unwrap()).collect();
        updates.push(us);
    }

    (restrictions, updates)
}
