use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    part1();
    part2();
}

fn parse() -> (Vec<u32>, Vec<u32>) {
    let mut list1 = Vec::with_capacity(INPUT.len());
    let mut list2 = Vec::with_capacity(INPUT.len());

    for line in INPUT.lines() {
        list1.push(line[..5].parse().unwrap());
        list2.push(line[8..].parse().unwrap());
    }

    (list1, list2)
}

fn part1() {
    let (mut list1, mut list2) = parse();
    list1.sort();
    list2.sort();

    let total_dist: u32 = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();

    println!("{}", total_dist)
}

fn part2() {
    let (list1, list2) = parse();

    let occurances_list2 = list2.iter().fold(HashMap::new(), |mut acc, &x| {
        acc.entry(x).and_modify(|e| *e += 1).or_insert(1);
        acc
    });

    let total_dist: u32 = list1
        .iter()
        .map(|x| x * occurances_list2.get(x).unwrap_or(&0))
        .sum();

    println!("{}", total_dist)
}
