use std::time::Instant;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let now = Instant::now();
    let input: Vec<Vec<u8>> = INPUT.lines().map(|line| line.bytes().collect()).collect();
    let p1 = part1(&input);
    let p2 = part2(&input);
    let elapsed = now.elapsed();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Elapsed: {:?}", elapsed);
}

fn part1(input: &[Vec<u8>]) -> u32 {
    let needle = [b'X', b'M', b'A', b'S'];
    let needle_rev = [b'S', b'A', b'M', b'X'];

    let mut count = 0;

    for h in 0..input.len() {
        for w in 0..input[h].len() {
            let current = input[h][w];
            if !(current == b'X' || current == b'S') {
                continue;
            }

            let l_to_r = range_left_to_right(input, h, w);
            let t_to_b = range_top_to_bottom(input, h, w);
            let tl_to_br = range_diagonal_tl_to_br(input, h, w);
            let tr_to_bl = range_diagonal_tr_to_bl(input, h, w);

            let match_l_to_r = matches(&l_to_r, &needle) || matches(&l_to_r, &needle_rev);
            let match_t_to_b = matches(&t_to_b, &needle) || matches(&t_to_b, &needle_rev);
            let match_tl_to_br = matches(&tl_to_br, &needle) || matches(&tl_to_br, &needle_rev);
            let match_tr_to_bl = matches(&tr_to_bl, &needle) || matches(&tr_to_bl, &needle_rev);

            count += match_l_to_r as u32
                + match_t_to_b as u32
                + match_tl_to_br as u32
                + match_tr_to_bl as u32;
        }
    }

    count
}

fn part2(input: &[Vec<u8>]) -> u32 {
    let needle = [b'M', b'A', b'S'];
    let needle_rev = [b'S', b'A', b'M'];

    let mut count = 0;
    for h in 1..input.len() - 1 {
        for w in 1..input[h].len() - 1 {
            if input[h][w] != b'A' {
                continue;
            }

            let tl_to_br = range_diagonal_tl_to_br(input, h - 1, w - 1);
            let tr_to_bl = range_diagonal_tr_to_bl(input, h - 1, w + 1);

            let is_lr_match = matches(&tl_to_br, &needle) || matches(&tl_to_br, &needle_rev);
            let is_rl_match = matches(&tr_to_bl, &needle) || matches(&tr_to_bl, &needle_rev);

            if is_lr_match && is_rl_match {
                count += 1;
            }
        }
    }

    count
}

fn matches<const N: usize>(haystack: &Option<[u8; N]>, needle: &[u8; N]) -> bool {
    haystack.map_or(false, |haystack| haystack == *needle)
}

fn range_left_to_right<const N: usize>(
    input_array: &[Vec<u8>],
    x: usize,
    y: usize,
) -> Option<[u8; N]> {
    let mut output = [0; N];
    for (idx, val) in output.iter_mut().enumerate() {
        *val = *input_array.get(x)?.get(y + idx)?;
    }
    Some(output)
}

fn range_top_to_bottom<const N: usize>(
    input_array: &[Vec<u8>],
    x: usize,
    y: usize,
) -> Option<[u8; N]> {
    let mut output = [0; N];
    for (idx, val) in output.iter_mut().enumerate() {
        *val = *input_array.get(x + idx)?.get(y)?;
    }
    Some(output)
}

fn range_diagonal_tl_to_br<const N: usize>(
    input_array: &[Vec<u8>],
    x: usize,
    y: usize,
) -> Option<[u8; N]> {
    let mut output = [0; N];
    for (idx, val) in output.iter_mut().enumerate() {
        *val = *input_array.get(x + idx)?.get(y + idx)?;
    }
    Some(output)
}

fn range_diagonal_tr_to_bl<const N: usize>(
    input_array: &[Vec<u8>],
    x: usize,
    y: usize,
) -> Option<[u8; N]> {
    if y < N - 1 {
        return None;
    }

    let mut output = [0; N];
    for (idx, val) in output.iter_mut().enumerate() {
        *val = *input_array.get(x + idx)?.get(y - idx)?;
    }
    Some(output)
}
