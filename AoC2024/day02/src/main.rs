use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    part1();
    part2();
}

fn part1() {
    let num_ok = INPUT
        .lines()
        .filter(|line| {
            let nums = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect_vec();

            check_levels(&nums)
        })
        .count();

    println!("part 1: {}", num_ok)
}

fn part2() {
    let num_ok = INPUT
        .lines()
        .filter(|line| {
            let nums: Vec<u32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            check_levels(&nums)
                || (0..nums.len()).any(|i| {
                    let mut modified_numbers = nums.clone();
                    modified_numbers.remove(i);
                    check_levels(&modified_numbers)
                })
        })
        .count();

    println!("part 2: {}", num_ok)
}

fn check_levels(nums: &[u32]) -> bool {
    let ordering = nums[0].cmp(&nums[1]);
    nums.iter()
        .tuple_windows()
        .all(|(a, b)| ordering == a.cmp(&b) && (1..=3).contains(&a.abs_diff(*b)))
}