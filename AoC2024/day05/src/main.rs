use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let now = std::time::Instant::now();
    let (r, u) = parse();
    let part1 = part1(&r, &u);
    let part2 = part2(&r, &u);
    let elapsed = now.elapsed();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Elapsed: {:?}", elapsed);
}

fn part1(restrictions: &HashSet<(usize, usize)>, updates: &[Vec<usize>]) -> usize {
    updates
        .iter()
        .filter_map(|update| {
            let page_map = page_index_map(update);
            if is_valid_update(restrictions, &page_map) {
                Some(update[update.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

fn part2(restrictions: &HashSet<(usize, usize)>, updates: &[Vec<usize>]) -> usize {
    updates
        .iter()
        .filter_map(|update| {
            let page_map = page_index_map(update);
            if !is_valid_update(restrictions, &page_map) {
                let sorted_update = sort_by_restrictions(update, restrictions);
                Some(sorted_update[sorted_update.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

fn page_index_map(update: &[usize]) -> HashMap<usize, usize> {
    update
        .iter()
        .enumerate()
        .map(|(index, &value)| (value, index))
        .collect()
}

fn is_valid_update(
    restrictions: &HashSet<(usize, usize)>,
    page_index_map: &HashMap<usize, usize>,
) -> bool {
    restrictions.iter().all(|&(r_first, r_second)| {
        let idx_first = page_index_map.get(&r_first);
        let idx_second = page_index_map.get(&r_second);
        if let (Some(idx_first), Some(idx_second)) = (idx_first, idx_second) {
            if idx_first > idx_second {
                return false;
            }
        }
        true
    })
}

fn sort_by_restrictions(update: &[usize], restrictions: &HashSet<(usize, usize)>) -> Vec<usize> {
    let mut sorted = update.to_vec();
    let mut swapped = true;

    while swapped {
        swapped = false;
        for (r_first, r_second) in restrictions {
            let idx_first = sorted.iter().position(|&x| x == *r_first);
            let idx_second = sorted.iter().position(|&x| x == *r_second);

            if let (Some(idx_first), Some(idx_second)) = (idx_first, idx_second) {
                if idx_first > idx_second {
                    sorted.swap(idx_first, idx_second);
                    swapped = true;
                }
            }
        }
    }

    sorted
}

fn parse() -> (HashSet<(usize, usize)>, Vec<Vec<usize>>) {
    let (restrictions_str, updates_str) = INPUT.split_once("\r\n\r\n").unwrap();

    let mut restr = HashSet::new();
    for line in restrictions_str.lines() {
        let r = line
            .split_once("|")
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .unwrap();
        restr.insert(r);
    }

    let mut updates = Vec::new();
    for update in updates_str.lines() {
        let us = update.split(',').map(|e| e.parse().unwrap()).collect();
        updates.push(us);
    }

    (restr, updates)
}
