use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

use itertools::Itertools;
use simple_grid::{Grid, GridIndex};

use crate::shared::point::Point;

fn parse_input(input: &str) -> Grid<char> {
    let height = input.lines().count();
    let width = input.lines().peekable().peek().expect("no line").len();
    Grid::new(
        width,
        height,
        input.lines().flat_map(|line| line.chars()).collect_vec(),
    )
}
fn get_neighbors(pos: GridIndex, grid: &Grid<char>, plant: char) -> Vec<GridIndex> {
    grid.cardinal_neighbor_indices_of(pos)
        .filter(|&idx| grid.get(idx).is_some_and(|&p| p == plant))
        .collect_vec()
}
fn bfs(
    pos: GridIndex,
    visited: &mut HashSet<GridIndex>,
    grid: &Grid<char>,
) -> (HashSet<GridIndex>, usize) {
    let mut queue = VecDeque::new();
    let plant = grid.get(pos).unwrap();
    let mut area = HashSet::default();
    let mut perimeter = 0;
    visited.insert(pos);
    queue.push_back(pos);
    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        area.insert(curr);
        let neighbors = get_neighbors(curr, grid, *plant);
        perimeter += 4 - neighbors.len(); // gets only exterior edges
        for nbr in neighbors {
            if !visited.contains(&nbr) {
                visited.insert(nbr);
                queue.push_back(nbr);
            }
        }
    }
    (area, perimeter)
}

fn get_price_and_regions(grid: &Grid<char>, regions: &mut Vec<HashSet<Point>>) -> usize {
    let mut visited = HashSet::default();
    let mut price = 0;
    grid.columns()
        .cartesian_product(&grid.rows().collect_vec())
        .map(|(x, &y)| GridIndex::from((x, y)))
        .for_each(|idx| {
            if !visited.contains(&idx) {
                let (area, perim) = bfs(idx, &mut visited, grid);
                price += area.len() * perim;
                regions.push(area.iter().map(|&idx| Point::from(idx)).collect());
            }
        });
    price
}

fn count_region_sides(region: &HashSet<Point>) -> usize {
    let mut side_count = 0;
    for direction in Point::new(0, 0).cardinal_neighbors() {
        let mut sides = HashSet::new();
        for pos in region {
            let pt = *pos + direction;
            if !region.contains(&pt) {
                sides.insert(pt);
            }
        }
        let mut to_remove = HashSet::new();
        for side in &sides {
            let mut next = (side.x + direction.y, side.y + direction.x).into();
            while sides.contains(&next) {
                to_remove.insert(next);
                next = (next.x + direction.y, next.y + direction.x).into();
            }
        }
        side_count += sides.len() - to_remove.len();
    }
    side_count
}

pub fn solve() {
    let input = read_to_string("inputs/day15.txt").expect("file read err");
    let grid = parse_input(&input);
    let mut regions = Vec::new();
    let price_pt1 = get_price_and_regions(&grid, &mut regions);
    println!("Part 1: {}", price_pt1);
    let price_pt2: usize = regions
        .iter_mut()
        .map(|region| region.len() * count_region_sides(region))
        .sum();
    println!("Part 2: {}", price_pt2);
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    #[test]
    fn test_parse() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(grid.get(GridIndex::from((1, 2))), Some(&'V'));
    }

    #[test]
    fn test_bfs_part1() {
        let grid = parse_input(TEST_INPUT);
        assert_eq!(get_price_and_regions(&grid, &mut vec![]), 1930);
    }

    #[test]
    fn test_bfs_part2() {
        let grid = parse_input(TEST_INPUT);
        let mut regions = Vec::new();
        _ = get_price_and_regions(&grid, &mut regions);
        let price_r_region = regions
            .into_iter()
            .find(|r| {
                let mut peekable = r.iter().peekable();
                let pt = peekable.peek().unwrap();
                let idx: GridIndex = (pt.x as usize, pt.y as usize).into();
                let c = grid.get(idx);
                c == Some(&'R')
            })
            .unwrap();
        assert_eq!(count_region_sides(&price_r_region), 10);
    }

    #[test]
    fn test_part2_total() {
        let grid = parse_input(TEST_INPUT);
        let mut regions = Vec::new();
        _ = get_price_and_regions(&grid, &mut regions);
        assert_eq!(
            1206,
            regions
                .iter_mut()
                .map(|region| region.len() * count_region_sides(region))
                .sum::<usize>()
        )
    }
}
