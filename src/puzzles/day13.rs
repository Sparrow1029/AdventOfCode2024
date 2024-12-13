use std::fs::read_to_string;

use regex::Regex;

type Point = (f64, f64);

fn parse_input(input: &str) -> Vec<(Point, Point, Point)> {
    let re = Regex::new(r"[A-Z\s:\+=]+(?<x>\d+),\sY[\+\=](?<y>\d+)").expect("invalid regex");
    input
        .split("\n\n")
        .map(|section| {
            section
                .lines()
                .filter_map(|line| {
                    let (_, [x, y]) = re.captures(line).map(|caps| caps.extract())?;
                    Some((x.parse().unwrap(), y.parse().unwrap()))
                })
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1], v[2]))
        .collect()
}

fn get_tokens(scenarios: &[(Point, Point, Point)], pos_mod: f64) -> f64 {
    let mut total = 0.0;
    for (a, b, prize) in scenarios {
        let ((ax, ay), (bx, by), (px, py)) = (a, b, prize);
        let a_perp_dot_p = ay * (px + pos_mod) - ax * (py + pos_mod);
        let a_perp_dot_b = ay * bx - ax * by;
        let (y, remainder) = (a_perp_dot_p / a_perp_dot_b, a_perp_dot_p % a_perp_dot_b);
        if remainder == 0.0 {
            let x = ((px + pos_mod) - bx * y) / ax;
            if x.fract() == 0.0 {
                total += 3.0 * x + y;
            }
        }
    }
    total
}

pub fn solve() {
    let input = read_to_string("inputs/day13.txt").expect("error reading file");
    let scenarios = parse_input(&input);
    println!("Part 1: {}", get_tokens(&scenarios, 0.0));
    println!("Part 2: {}", get_tokens(&scenarios, 10_000_000_000_000.0));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_parse() {
        let scenarios = parse_input(TEST_INPUT);
        assert_eq!(scenarios.len(), 4);
    }
    #[test]
    fn test_solve() {
        let scenarios = parse_input(TEST_INPUT);
        let part1 = get_tokens(&scenarios, 0.0);
        assert_eq!(part1, 480.0);
    }
}
