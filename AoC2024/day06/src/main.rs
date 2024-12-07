use ahash::AHashSet;
use bitvec::vec::BitVec;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

const INPUT: &str = include_str!("input.txt");

const OBSTACLE: bool = true;
const EMPTY: bool = false;

#[derive(Debug)]
struct TimeLoopError;

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
            Direction::Up if x > 0 => Some((x - 1, y)),
            Direction::Right => Some((x, y + 1)),
            Direction::Down => Some((x + 1, y)),
            Direction::Left if y > 0 => Some((x, y - 1)),
            _ => None,
        }
    }
}

fn main() {
    let t0 = std::time::Instant::now();
    let (grid, start) = parse_input(INPUT);
    let part1 = part1(&grid, start);
    let part2 = part2(&grid, start);
    println!("Time: {:?}", t0.elapsed());
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn part1(input: &[BitVec], start: (usize, usize)) -> usize {
    walk(start.0, start.1, Direction::Up, input)
        .expect("Time loop in initial walk")
        .iter()
        .map(|(x, y, _)| (*x, *y))
        .unique()
        .count()
}

fn part2(input: &[BitVec], start: (usize, usize)) -> usize {
    let guard_positions =
        walk(start.0, start.1, Direction::Up, input).expect("Time loop in initial walk");

    let possible_obstacle_positions = guard_positions
        .iter()
        .map(|(x, y, _)| (*x, *y))
        .filter(|(x, y)| *x != start.0 || *y != start.1)
        .unique()
        .collect_vec();

    possible_obstacle_positions
        .into_par_iter()
        .filter(|(x, y)| {
            let mut new_grid = input.to_owned();
            new_grid[*x].set(*y, true);
            walk(start.0, start.1, Direction::Up, &new_grid).is_err()
        })
        .count()
}

fn walk(
    start_x: usize,
    start_: usize,
    start_dir: Direction,
    grid: &[BitVec],
) -> Result<AHashSet<(usize, usize, Direction)>, TimeLoopError> {
    let mut dir = start_dir;
    let mut pos = (start_x, start_);

    let mut visited = AHashSet::new();
    visited.insert((start_x, start_, start_dir));

    loop {
        // out of bounds
        let Some((look_at_h, look_at_w)) = dir.mov(pos.0, pos.1) else {
            break;
        };
        if look_at_h >= grid.len() || look_at_w >= grid[0].len() {
            break;
        }

        (pos, dir) = step(grid, look_at_h, look_at_w, dir, pos);

        if visited.contains(&(pos.0, pos.1, dir)) {
            return Err(TimeLoopError);
        }

        visited.insert((pos.0, pos.1, dir));
    }
    Ok(visited)
}

fn step(
    grid: &[BitVec],
    look_at_h: usize,
    look_at_w: usize,
    mut dir: Direction,
    mut pos: (usize, usize),
) -> ((usize, usize), Direction) {
    match grid[look_at_h][look_at_w] {
        OBSTACLE => dir = dir.next(),
        EMPTY => pos = (look_at_h, look_at_w),
    }
    (pos, dir)
}

fn parse_input(input: &str) -> (Vec<BitVec>, (usize, usize)) {
    let mut guard_pos = None;
    let mut grid: Vec<BitVec> = vec![];
    for line in input.lines() {
        let row = line
            .chars()
            .enumerate()
            .map(|(idx, c)| match c {
                '.' => EMPTY,
                '#' => OBSTACLE,
                '^' => {
                    guard_pos = Some((grid.len(), idx));
                    EMPTY
                }
                _ => unreachable!(),
            })
            .collect();
        grid.push(row);
    }
    (grid, guard_pos.unwrap())
}
