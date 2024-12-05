use std::{cmp::Ordering, fs::read_to_string};

use nom::{bytes::complete::tag, character, multi::separated_list1, IResult};
use simple_grid::Grid;

type Matrix = Grid<Ordering>;

fn parse_input(input: &str) -> (Vec<(u8, u8)>, Vec<Vec<u8>>) {
    let split: Vec<&str> = input.split("\n\n").collect();
    let (pair_str, page_str) = (split[0], split[1]);
    let pairs = parse_pairs(pair_str);
    let pages = parse_pages(page_str);
    (pairs, pages)
}

fn parse_pairs(input: &str) -> Vec<(u8, u8)> {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once('|').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn parse_pages(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| -> Vec<u8> {
            if let Ok((_, digits)) = parse_digits(l) {
                digits
            } else {
                panic!("ah no")
            }
        })
        .collect()
}

fn parse_digits(s: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(tag(","), character::complete::u8)(s)
}

fn create_matrix(pairs: &[(u8, u8)]) -> Matrix {
    let max = *pairs
        .iter()
        .flat_map(|(a, b)| [a, b])
        .max()
        .expect("iterator is non-empty") as usize
        + 1;
    let mut matrix = Grid::new(max, max, vec![Ordering::Equal; max * max]);
    for (a, b) in pairs {
        matrix.replace_cell((*a as usize, *b as usize), Ordering::Less);
        matrix.replace_cell((*b as usize, *a as usize), Ordering::Greater);
    }
    matrix
}

fn valid_pages(pages: &[Vec<u8>], matrix: &Matrix) -> usize {
    pages
        .iter()
        .filter_map(|p| {
            if check_order(p, matrix) {
                Some(p[p.len() / 2] as usize)
            } else {
                None
            }
        })
        .sum()
}

fn fix_invalid_pages(pages: &mut [Vec<u8>], matrix: &Matrix) -> usize {
    pages
        .iter_mut()
        .filter(|p| !check_order(p, matrix))
        .map(|v| {
            v.sort_by(|a, b| *matrix.get((*a as usize, *b as usize)).unwrap());
            v[v.len() / 2] as usize
        })
        .sum()
}

fn check_order(page: &[u8], matrix: &Matrix) -> bool {
    page.is_sorted_by(|a, b| *matrix.get((*a as usize, *b as usize)).unwrap() == Ordering::Less)
}

pub fn solve() {
    let input = read_to_string("inputs/day05.txt").expect("read file");
    let (pairs, mut pages) = parse_input(&input);
    let matrix = create_matrix(&pairs);
    println!("Part 1: {}", valid_pages(&pages, &matrix));
    println!("Part 2: {}", fix_invalid_pages(&mut pages, &matrix));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    fn get_data() -> (Vec<(u8, u8)>, Vec<Vec<u8>>) {
        let split: Vec<&str> = TEST_INPUT.split("\n\n").collect();
        let (pair_str, page_str) = (split[0], split[1]);
        let pairs = parse_pairs(pair_str);
        let pages = parse_pages(page_str);
        (pairs, pages)
    }

    #[test]
    fn test_parse() {
        let (pairs, pages) = get_data();
        assert_eq!((53, 13), *pairs.last().unwrap());
        assert_eq!(6, pages.len());
    }

    #[test]
    fn test_create_matrix() {
        let (pairs, _) = get_data();
        let matrix = create_matrix(&pairs);
        assert_eq!(Some(&Ordering::Less), matrix.get((97, 29)));
    }

    #[test]
    fn test_page_order() {
        let (pairs, pages) = get_data();
        let matrix = create_matrix(&pairs);
        assert_eq!(143usize, valid_pages(&pages, &matrix));
    }

    #[test]
    fn test_fix_unordered() {
        let (pairs, mut pages) = get_data();
        let matrix = create_matrix(&pairs);
        assert_eq!(123usize, fix_invalid_pages(&mut pages, &matrix))
    }
}
