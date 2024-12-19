use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

use itertools::Itertools;
use simple_grid::{Grid, GridIndex};

use crate::shared::point::Point;

fn parse_input(input: &str) -> Grid<char> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();
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

/// Get all sides by checking one direction at a time
/// ```no_run
/// (0, -1), (1, 0), (0, 1) (-1, 0)
///   up     right    down   left
/// ```
/// Example for 'R':
/// ```no_run
/// RRRRII we check first  ^^^^II
/// RRRRII by looking up   ^^^^II
/// VVRRRC like so:        VV^^^C
/// VVRCCC                 VV^CCC
/// VVVVCJ                 VVVVCJ
/// ```
///
/// We record all positions that are not in the region
/// ```no_run
/// (0, -1), (1, -1), (2, -1), (3, -1), (4, 2)
/// ```
/// Then, for each of those positions, we collect sides facing up by looking
/// counter-clockwise to the current direction, and removing all but the top
/// corner of a face/side.
/// ```no_run
/// to_remove = /* HashSet */ [(0, -1), (1, -1), (2, -1)];
/// sides.len() /* 5 */ - remove.len() /* 3 */ = 2;
/// ```
/// so we have collected the two top-facing sides, and move on to the next
fn count_region_sides(region: &HashSet<Point>) -> usize {
    let mut side_count = 0;
    for direction in Point::new(0, 0).cardinal_neighbors() {
        // println!("Direction: {direction}");
        let mut sides = HashSet::new();
        for pos in region {
            let pt = *pos + direction;
            if !region.contains(&pt) {
                sides.insert(pt);
            }
        }
        let mut to_remove = HashSet::new();
        log::debug!(
            "all side pieces: {}",
            sides
                .iter()
                .sorted_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)))
                .map(|s| format!("{s}"))
                .collect_vec()
                .join(" ")
        );
        for side in sides
            .iter()
            .sorted_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)))
        {
            log::debug!("  checking side: {side}");
            let mut next = (side.x + direction.y, side.y + direction.x).into();
            while sides.contains(&next) {
                log::debug!("    next: {next}");
                to_remove.insert(next);
                next = (next.x + direction.y, next.y + direction.x).into();
            }
        }
        log::debug!("  to_remove: {to_remove:?}");
        side_count += sides.len() - to_remove.len();
    }
    side_count
}

pub fn solve() {
    let input = read_to_string("inputs/day12.txt").expect("file read err");
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
    use crate::shared::util::test_setup;

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
        test_setup();
        let grid = parse_input(TEST_INPUT);
        assert_eq!(grid.get(GridIndex::from((1, 2))), Some(&'V'));
    }

    #[test]
    fn test_bfs_part1() {
        test_setup();
        let grid = parse_input(TEST_INPUT);
        assert_eq!(get_price_and_regions(&grid, &mut vec![]), 1930);
    }

    #[test]
    fn test_bfs_part2() {
        test_setup();
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
        test_setup();
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
