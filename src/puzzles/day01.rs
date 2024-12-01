use std::ops::Mul;

use crate::shared::util::read_lines;

fn parse() -> (Vec<usize>, Vec<usize>) {
    let mut one = vec![];
    let mut two = vec![];
    for l in read_lines("inputs/day01.txt") {
        let mut split = l.split_whitespace();
        one.push(split.next().unwrap().parse::<usize>().unwrap());
        two.push(split.next().unwrap().parse::<usize>().unwrap());
    }
    one.sort();
    two.sort();
    (one, two)
}

fn part1(a: &[usize], b: &[usize]) {
    let result: usize = a.iter().zip(b.iter()).map(|(x, y)| x.abs_diff(*y)).sum();
    println!("Part1: {result}");
}

fn part2(a: &[usize], b: &[usize]) {
    let result: usize = a
        .iter()
        .map(|x| x.mul(b.iter().filter(|i| *i == x).count()))
        .sum();
    println!("Part2: {result}");
}

pub fn solve() {
    let (a, b) = parse();
    part1(&a, &b);
    part2(&a, &b);
}
