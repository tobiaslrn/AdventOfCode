use regex::Regex;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

fn part_1() -> usize {
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .unwrap()
        .captures_iter(INPUT)
        .map(|m| m.extract())
        .map(|(_, [a, b])| {
            let a: usize = a.parse().unwrap();
            let b: usize = b.parse().unwrap();
            a * b
        })
        .sum()
}

fn part_2() -> usize {
    let mut enabled = true;
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)")
        .unwrap()
        .captures_iter(INPUT)
        .map(|m| match m.get(0).unwrap().as_str() {
            "do()" => {
                enabled = true;
                0
            }
            "don't()" => {
                enabled = false;
                0
            }
            _ if enabled => {
                let a: usize = m.get(1).unwrap().as_str().parse().unwrap();
                let b: usize = m.get(2).unwrap().as_str().parse().unwrap();
                a * b
            }
            _ => 0,
        })
        .sum()
}
