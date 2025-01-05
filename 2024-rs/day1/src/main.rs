use std::collections::HashMap;

fn main() {
    lib::submit_p1(p1);
    lib::submit_p2(p2);
}

fn p1(input: &str) -> i64 {
    let (first, second) = parse(input);

    first
        .iter()
        .zip(second)
        .map(|(a, b)| (a - b).abs())
        .sum::<i64>()
}

fn p2(input: &str) -> i64 {
    let (first, second) = parse(input);

    let counts = second.iter().fold(HashMap::new(), |mut map, &x| {
        *map.entry(x).or_insert(0) += 1;
        map
    });

    first
        .iter()
        .map(|&x| x * counts.get(&x).unwrap_or(&0))
        .sum()
}

fn parse(input: &str) -> (Vec<i64>, Vec<i64>) {
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

    (first, second)
}
