use itertools::{repeat_n, Itertools};
use std::{collections::HashMap, fs::read_to_string};

fn parse_input(input: &str) -> HashMap<usize, Vec<usize>> {
    input
        .lines()
        .filter_map(|l| l.split_once(':'))
        .map(|(val, rest)| {
            let digits = rest
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            (val.parse().unwrap(), digits)
        })
        .collect()
}

/// For each `key: [value]` pair in the input map, we get every possible permutation of `['+', 'x',
/// '|']` operators and try them in order. If any series of operations performed on the vec of
/// `[value, ...]`s produce the `k`, we keep the `k` and sum the resulting `vec[k]`.
/// Found a neat trick to use the base 10 logarithm of a number to get the number of digits:
/// ```
/// //Ex: x = 45, acc = 123
/// let num_digits = x.checked_ilog10().unwrap() + 1 // 2
/// let multiplier = 10usize.pow(num_digits) // 100
/// acc * multiplier /*12300*/ + 45 == 12345
/// ```
fn calibrate(map: &HashMap<usize, Vec<usize>>, ops: &[char]) -> usize {
    map.iter()
        .filter_map(|(k, v)| {
            repeat_n(ops, v.len() - 1)
                .multi_cartesian_product()
                .any(|combo| {
                    let start = v.first().unwrap();
                    v.iter()
                        .skip(1) // skip first digit, since we use it as start for acc
                        .zip(&combo)
                        .fold(*start, |acc, (x, op)| match op {
                            'x' => acc * x,
                            '+' => acc + x,
                            '|' => acc * 10usize.pow(x.checked_ilog10().unwrap() + 1) + x,
                            _ => panic!("invalid operation"),
                        })
                        == *k
                })
                .then_some(k)
        })
        .sum()
}

pub fn solve() {
    let input = read_to_string("inputs/day07.txt").expect("file open error");
    let map = parse_input(&input);
    println!("Part 1: {}", calibrate(&map, &['+', 'x']));
    println!("Part 2: {}", calibrate(&map, &['+', 'x', '|']));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_parse() {
        let map = parse_input(TEST_INPUT);
        assert_eq!(27, map.values().flatten().count());
    }

    #[test]
    fn test_part1() {
        let map = parse_input(TEST_INPUT);
        assert_eq!(3749, calibrate(&map, &['+', 'x']));
    }

    #[test]
    fn test_part2() {
        let map = parse_input(TEST_INPUT);
        assert_eq!(11387, calibrate(&map, &['+', 'x', '|']));
    }
}
