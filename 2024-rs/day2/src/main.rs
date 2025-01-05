fn main() {
    lib::submit_p1(p1);
    lib::submit_p2(p2);
}

fn p1(input: &str) -> i64 {
    let reports = parse_reports(input);

    reports.iter().filter(|report| is_safe(report)).count() as i64
}

fn p2(input: &str) -> i64 {
    let reports = parse_reports(input);

    reports
        .iter()
        .filter(|report| is_safe_with_tolerance(report))
        .count() as i64
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Trend {
    Increasing,
    Decreasing,
}

fn parse_reports(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().expect("Failed to parse the input"))
                .collect()
        })
        .collect()
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

#[cfg(test)]
mod test {
    use crate::{p1, p2};

    #[test]
    fn test_part1() {
        let input = lib::get_input_dbg();

        let result = p1(&input);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2() {
        let input = lib::get_input_dbg();

        let result = p2(&input);

        assert_eq!(result, 2);
    }
}
