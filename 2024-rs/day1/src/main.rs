use std::{collections::HashMap, time::Instant};

use clap::Parser;

fn main() {
    let args = lib::Args::parse();
    let input = lib::get_input(&args);

    let start = Instant::now();

    let (mut first, mut second): (Vec<i64>, Vec<i64>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let first: i64 = parts
                .next()
                .expect("Missing first number")
                .parse()
                .expect("Failed to parse first number");
            let second: i64 = parts
                .next()
                .expect("Missing second number")
                .parse()
                .expect("Failed to parse second number");
            (first, second)
        })
        .unzip();

    first.sort();
    second.sort();

    match args.part {
        1 => {
            let result = p1(&first, &second);
            let end = start.elapsed().as_micros();
            println!("Result for part 1: {result} (🚀 {end} us)");
        }
        2 => {
            let result = p2(&first, &second);
            let end = start.elapsed().as_micros();
            println!("Result for part 2: {result} (🚀 {end} us)");
        }
        _ => unreachable!(),
    }
}

fn p1(first: &[i64], second: &[i64]) -> i64 {
    first
        .iter()
        .zip(second)
        .map(|(a, b)| (a - b).abs())
        .sum::<i64>()
}

fn p2(first: &[i64], second: &[i64]) -> i64 {
    let counts = second.iter().fold(HashMap::new(), |mut map, &x| {
        *map.entry(x).or_insert(0) += 1;
        map
    });

    first
        .iter()
        .map(|&x| x * counts.get(&x).unwrap_or(&0))
        .sum()
}
