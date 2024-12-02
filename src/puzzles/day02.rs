use std::cmp::Ordering;

use nom::{
    character::complete::{space1, u32},
    multi::separated_list1,
    IResult,
};

use crate::shared::util::read_lines;

fn parse() -> Vec<Vec<u32>> {
    read_lines("inputs/day02.txt")
        .iter()
        .map(|s| match parse_line(s) {
            Ok((_, digits)) => digits,
            Err(e) => panic!("Error reading input: {e:?}"),
        })
        .collect::<Vec<_>>()
}

fn parse_line(s: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, u32)(s)
}

fn is_safe(report: &[u32]) -> bool {
    let ordering = report.first().unwrap().cmp(report.last().unwrap());
    // exit early if the first/last are equal
    if ordering == Ordering::Equal {
        return false;
    }
    for pair in report.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        if !(a.cmp(&b) == ordering && (1..=3).contains(&(a.abs_diff(b)))) {
            return false;
        }
    }
    true
}

pub fn solve() {
    let input = parse();
    part1(&input);
    part2(&input);
}

fn part1(input: &[Vec<u32>]) {
    let safe_count = input.iter().filter(|report| is_safe(report)).count();
    println!("Part 1: {safe_count}");
}

fn part2(input: &[Vec<u32>]) {
    let mut safe_count = 0;
    for report in input {
        if is_safe(report) || remove_one(report) {
            safe_count += 1;
        }
    }
    println!("Part 2: {safe_count}");
}

fn remove_one(report: &[u32]) -> bool {
    (0..report.len()).any(|i| {
        let removed = report[..i]
            .iter()
            .cloned()
            .chain(report[i + 1..].iter().cloned())
            .collect::<Vec<_>>();
        is_safe(&removed)
    })
}

#[cfg(test)]
mod test {
    use super::{is_safe, remove_one};

    const DIGITS: [[u32; 5]; 6] = [
        [7, 6, 4, 2, 1],
        [1, 2, 7, 8, 9],
        [9, 7, 6, 2, 1],
        [1, 3, 2, 4, 5],
        [8, 6, 4, 4, 1],
        [1, 3, 6, 7, 9],
    ];

    #[test]
    fn test_part1() {
        let result = DIGITS
            .into_iter()
            .map(|arr| is_safe(&arr))
            .collect::<Vec<bool>>();
        let expected = [true, false, false, false, false, true];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let result = DIGITS
            .into_iter()
            .map(|arr| is_safe(&arr) || remove_one(&arr))
            .collect::<Vec<bool>>();
        let expected = [true, false, false, true, true, true];
        assert_eq!(result, expected);
    }
}
