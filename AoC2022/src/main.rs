use aoc1::Day1;
use aoc2::Day2;
use aoc3::Day3;
use aoc4::Day4;
use aoc5::Day5;
use aoc6::Day6;
use std::{
    time::{Duration, Instant},
    vec,
};
pub mod aoc1;
pub mod aoc2;
pub mod aoc3;
pub mod aoc4;
pub mod aoc5;
pub mod aoc6;
pub mod aoc7;
use crate::aoc7::Day7;
use comfy_table::{
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
    presets::UTF8_FULL,
    Table,
};

const BENCHMARK: bool = true;

fn main() {
    if BENCHMARK {
        benchmark_all();
    } else {
        let now = Instant::now();
        let part = Day2::default();
        print!("{}", part.p2());
        println!("\n{:?}", now.elapsed())
    }
}

fn benchmark_all() {
    let runs_per_x = 1000;
    let all_parts: Vec<Box<dyn Part>> = vec![
        Box::new(Day1::default()),
        Box::new(Day2::default()),
        Box::new(Day3::default()),
        Box::new(Day4::default()),
        Box::new(Day5::default()),
        Box::new(Day6::default()),
        Box::new(Day7::default()),
    ];

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Day", "Part 1", "Part 2", "Combined", "Relative"]);
    let mut total_duration_p1 = Duration::new(0, 0);
    let mut total_duration_p2 = Duration::new(0, 0);
    let mut time_data = Vec::new();
    for (day, part) in all_parts.iter().enumerate() {
        let now1 = Instant::now();
        for _ in 0..runs_per_x {
            part.p1();
        }
        let p1_elapsed = now1.elapsed() / runs_per_x;
        let now2 = Instant::now();
        for _ in 0..runs_per_x {
            part.p2();
        }
        let p2_elapsed = now2.elapsed() / runs_per_x;
        total_duration_p1 += p1_elapsed;
        total_duration_p2 += p2_elapsed;
        time_data.push((
            format!("{}", day + 1),
            format!("{:?}", p1_elapsed),
            format!("{:?}", p2_elapsed),
            p1_elapsed + p2_elapsed,
        ))
    }

    let combined_duration = total_duration_p1 + total_duration_p2;
    for data in time_data {
        let percentage = data.3.as_secs_f64() / combined_duration.as_secs_f64() * 100.0;
        table.add_row(vec![
            data.0,
            data.1,
            data.2,
            format!("{:#?}", data.3),
            format!("{:.1}%", percentage),
        ]);
    }

    table.add_row(vec![
        format!("Total"),
        format!("{:?}", total_duration_p1),
        format!("{:?}", total_duration_p2),
        format!("{:?}", combined_duration),
        "100%".to_string(),
    ]);

    println!("\n{table}");
}

pub trait Part {
    fn p1(&self) -> String;
    fn p2(&self) -> String;
}
