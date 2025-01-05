use regex::Regex;

fn main() {
    lib::submit_p1(p1);
    lib::submit_p2(p2);
}

fn p1(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input).fold(0, |acc, caps| {
        let x = caps[1].parse::<i64>().unwrap();
        let y = caps[2].parse::<i64>().unwrap();

        acc + x * y
    })
}

fn p2(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)").unwrap();

    let (sum, _) = re
        .captures_iter(input)
        .fold((0, true), |(acc, do_mul), caps| {
            match caps.get(0).map(|m| m.as_str()) {
                Some("do()") => (acc, true),
                Some("don't()") => (acc, false),
                Some(_) if do_mul => {
                    let x = caps
                        .get(1)
                        .and_then(|m| m.as_str().parse::<i64>().ok())
                        .unwrap();
                    let y = caps
                        .get(2)
                        .and_then(|m| m.as_str().parse::<i64>().ok())
                        .unwrap();
                    (acc + x * y, true)
                }
                _ => (acc, do_mul),
            }
        });

    sum
}

#[cfg(test)]
mod tests {
    use crate::{p1, p2};

    #[test]
    fn test_part1() {
        let input = lib::get_input_dbg();

        let result = p1(&input);

        assert_eq!(result, 161);
    }

    #[test]
    fn test_part2() {
        let input = lib::get_input_dbg();

        let result = p2(&input);

        assert_eq!(result, 48)
    }
}
