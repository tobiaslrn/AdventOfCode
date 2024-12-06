use std::collections::HashSet;

use bitvec::vec::BitVec;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let (grid, start) = parse_input(INPUT);
    let part1 = part1(&grid, start);
    println!("Part 1: {}", part1);
    let part2 = part2(&grid, start);
    println!("Part 2: {}", part2);
}

fn part1(input: &[BitVec], start: (usize, usize)) -> usize {
    let visited = walk(start.0, start.1, Direction::Up, input);
    visited
        .unwrap()
        .iter()
        .map(|(x, y, _)| (*x, *y))
        .collect::<HashSet<(usize, usize)>>()
        .len()
}

fn part2(input: &[BitVec], start: (usize, usize)) -> usize {
    let mut timeloops = 0;
    let visited = walk(start.0, start.1, Direction::Up, input).unwrap();

    for (x, y, direction) in visited.into_iter() {
        let Some((look_at_h, look_at_w)) = direction.mov(x, y) else {
            continue;
        };

        if look_at_h >= input.len() || look_at_w >= input[0].len() {
            continue;
        }

        let mut new_grid = input.to_vec();
        new_grid[look_at_h].set(look_at_w, true);

        if walk(start.0, start.1, Direction::Up, &new_grid).is_err() {
            timeloops += 1;
        }
    }

    timeloops
}

fn walk(
    start_x: usize,
    start_: usize,
    start_dir: Direction,
    input: &[BitVec],
) -> Result<HashSet<(usize, usize, Direction)>, ()> {
    let mut direction = start_dir;
    let mut current_pos = (start_x, start_);

    let mut visited = HashSet::new();
    visited.insert((start_x, start_, start_dir));

    loop {
        let Some((look_at_h, look_at_w)) = direction.mov(current_pos.0, current_pos.1) else {
            break;
        };

        if look_at_h >= input.len() || look_at_w >= input[0].len() {
            // guard is out of bounds
            break;
        }

        let looking_at = *input[look_at_h].get(look_at_w).unwrap();
        if looking_at {
            direction = direction.next();
        } else {
            current_pos = (look_at_h, look_at_w);
        }

        if visited.contains(&(current_pos.0, current_pos.1, direction)) {
            // guard is stuck in a loop
            return Err(());
        }

        visited.insert((current_pos.0, current_pos.1, direction));
    }
    Ok(visited)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn mov(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                if x == 0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
            Direction::Right => Some((x, y + 1)),
            Direction::Down => Some((x + 1, y)),
            Direction::Left => {
                if y == 0 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
        }
    }
}

fn parse_input(input: &str) -> (Vec<BitVec>, (usize, usize)) {
    let mut guard_pos = None;
    let mut grid: Vec<BitVec> = vec![];
    for line in input.lines() {
        let row = line
            .chars()
            .enumerate()
            .map(|(idx, c)| match c {
                '.' => false,
                '#' => true,
                '^' => {
                    guard_pos = Some((grid.len(), idx));
                    false
                }
                _ => unreachable!(),
            })
            .collect();
        grid.push(row);
    }
    (grid, guard_pos.unwrap())
}
