use ahash::AHashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse(INPUT);
    let t0 = std::time::Instant::now();
    let part1 = calculate_stones(&input, 25);
    let part2 = calculate_stones(&input, 75);
    println!("Time: {:?}", t0.elapsed());
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn calculate_stones(input: &Vec<usize>, blinks: usize) -> usize {
    let mut cache = AHashMap::new();
    input
        .iter()
        .map(|x| change_cached(*x, blinks, &mut cache))
        .sum()
}

fn change_cached(num: usize, depth: usize, cache: &mut AHashMap<(usize, usize), usize>) -> usize {
    if let Some(&cached_result) = cache.get(&(num, depth)) {
        return cached_result;
    }

    if depth == 0 {
        cache.insert((num, depth), 1);
        return 1;
    }

    let (p1, p2) = change_number(num);
    let mut result = change_cached(p1, depth - 1, cache);
    if let Some(p2) = p2 {
        result += change_cached(p2, depth - 1, cache);
    }

    cache.insert((num, depth), result);
    result
}

fn change_number(num: usize) -> (usize, Option<usize>) {
    match num {
        0 => (1, None),
        _ if count_digits(num) % 2 == 0 => {
            let (p1, p2) = split_number(num);
            (p1, Some(p2))
        }
        _ => (num * 2024, None),
    }
}

fn count_digits(n: usize) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

fn split_number(num: usize) -> (usize, usize) {
    let mut divisor = 1;
    let digit_count = count_digits(num);
    let split_position = digit_count / 2;

    for _ in 0..split_position {
        divisor *= 10;
    }

    (num / divisor, num % divisor)
}

fn parse(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}
