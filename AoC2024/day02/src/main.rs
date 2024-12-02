use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    part1();
    part2();
}

fn part1() {
    let num_ok = INPUT
        .lines()
        .filter(|line| check_levels(&parse_line(line)))
        .count();

    println!("part 1: {}", num_ok)
}

fn part2() {
    let num_ok = INPUT
        .lines()
        .filter(|line| {
            let levels = parse_line(line);
            check_levels(&levels)
                || (0..levels.len()).any(|i| {
                    let mut dampened_levels = levels.clone();
                    dampened_levels.remove(i);
                    check_levels(&dampened_levels)
                })
        })
        .count();

    println!("part 2: {}", num_ok)
}

fn parse_line(line: &&str) -> Vec<u32> {
    line.split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn check_levels(nums: &[u32]) -> bool {
    let ordering = nums[0].cmp(&nums[1]);
    nums.iter()
        .tuple_windows()
        .all(|(a, b)| ordering == a.cmp(&b) && (1..=3).contains(&a.abs_diff(*b)))
}
