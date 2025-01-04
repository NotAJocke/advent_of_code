use std::time::Instant;

use clap::Parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Trend {
    Increasing,
    Decreasing,
}

fn main() {
    let args = lib::Args::parse();
    let input = lib::get_input(&args);

    let start = Instant::now();

    let reports: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().expect("Failed to parse the input"))
                .collect()
        })
        .collect();

    match args.part {
        1 => {
            let result = p1(&reports);
            let end = start.elapsed().as_micros();
            println!("Result for part 1: {result} (🚀 {end} us)");
        }
        2 => {
            let result = p2(&reports);
            let end = start.elapsed().as_micros();
            println!("Result for part 2: {result} (🚀 {end} us)");
        }
        _ => unreachable!(),
    }
}

fn p1(reports: &[Vec<i64>]) -> i64 {
    reports.iter().filter(|report| is_safe(report)).count() as i64
}

fn p2(reports: &[Vec<i64>]) -> i64 {
    reports
        .iter()
        .filter(|report| is_safe_with_tolerance(report))
        .count() as i64
}

fn is_safe(report: &[i64]) -> bool {
    let trend = if report[0] < report[1] {
        Trend::Increasing
    } else {
        Trend::Decreasing
    };

    report.windows(2).all(|window| match window {
        [a, b] => {
            let diff = a - b;
            match trend {
                Trend::Increasing if diff > 0 => false,
                Trend::Decreasing if diff < 0 => false,
                _ if diff.abs() > 3 || diff.abs() < 1 => false,
                _ => true,
            }
        }
        _ => false,
    })
}

fn generate_variations<T: Clone>(vec: &[T]) -> Vec<Vec<T>> {
    (0..vec.len())
        .map(|i| {
            vec.iter()
                .enumerate()
                .filter_map(|(j, x)| if i == j { None } else { Some(x.clone()) })
                .collect()
        })
        .collect()
}

fn is_safe_with_tolerance(report: &[i64]) -> bool {
    if is_safe(report) {
        return true;
    }

    generate_variations(report).iter().any(|r| is_safe(r))
}
