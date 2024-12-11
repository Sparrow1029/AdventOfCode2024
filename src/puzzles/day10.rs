use std::collections::HashSet;

use simple_grid::{Grid, GridIndex};

fn dfs(
    grid: &Grid<u32>,
    visited: &mut HashSet<GridIndex>,
    start_pos: (usize, usize),
    total: &mut u32,
) {
    let cur_cell_val = grid.get(start_pos).unwrap();
    println!("dfs {start_pos:?} - {cur_cell_val}");
    for nbr in grid.cardinal_neighbor_indices_of(start_pos) {
        if visited.contains(&nbr) {
            continue;
        }
        let nbr_cell_val = grid.get(nbr).unwrap();
        // let seen = !visited.insert(nbr);
        let greater = nbr_cell_val > cur_cell_val;
        let is_one_diff = if greater {
            nbr_cell_val - cur_cell_val == 1
        } else {
            false
        };
        println!(
            "  checking ({}, {}) - {nbr_cell_val}",
            nbr.column(),
            nbr.row()
        );
        // println!("   seen? {seen:?}");
        // if !seen {
        //     println!(
        //         "   {nbr_cell_val} > {cur_cell_val}? {}",
        //         nbr_cell_val > cur_cell_val
        //     );
        //     if nbr_cell_val >= cur_cell_val {
        //         println!(
        //             "   {nbr_cell_val} - {cur_cell_val} == 1? {}",
        //             nbr_cell_val - cur_cell_val == 1
        //         );
        //     }
        // }
        if !greater || !is_one_diff {
            println!("  continuing!");
            continue;
        }
        if *nbr_cell_val == 9 {
            println!("FOUND 9!");
            *total += 1;
            return;
        }
        if !visited.insert(nbr) {
            panic!("already in there")
        }
        dfs(grid, visited, (nbr.column(), nbr.row()), total);
    }
}

fn trailheads(grid: &Grid<u32>) -> Vec<(usize, usize)> {
    grid.cells_with_indices_iter()
        .filter(|(_, cell)| *cell == &0)
        .map(|(i, _)| (i.column(), i.row()))
        .collect()
}

// fn parse_input(input: &str) -> Graph<u32, u32, petgraph::Undirected> {
fn parse_input(input: &str) -> Grid<u32> {
    let width = input.lines().next().unwrap().len();
    let height = input.len() / width;
    let grid = Grid::new(
        width,
        height,
        input
            .lines()
            .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap()))
            .collect(),
    );
    grid
    // let xy_to_idx = |x: usize, y: usize| -> u32 { (y * width + x) as u32 };
    // let mut graph = Graph::new_undirected();
    // for y in 0..height {
    //     for x in 0..width {
    //         let cur_cell = grid.get((x, y)).unwrap();
    //         let idx = xy_to_idx(x, y);
    //         let mut node = if let Some(n) = graph.ge
    //         for pt in grid.neighbor_indices_of((x, y)) {
    //             let nbr_cell = grid.get((pt.column(), pt.row())).unwrap();
    //             let nbr_idx = xy_to_idx(pt.column(), pt.row());
    //             let nbr_weight = grid.get(pt).unwrap().abs_diff(*cur_cell);
    //             if nbr_weight == 1 {
    //                 graph.add_edge(idx.into(), nbr_idx.into(), nbr_weight);
    //             }
    //         }
    //     }
    // }
    // println!("{graph:?}");
    // graph
}

fn part1() {}

fn part2() {}

pub fn solve() {}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_parse() {
        let grid = parse_input(TEST_INPUT);
        println!("{}", grid.to_pretty_string());
        let mut visited = HashSet::new();

        let mut total = 0;
        trailheads(&grid)
            .iter()
            .for_each(|pos| dfs(&grid, &mut visited, *pos, &mut total));
        println!("total: {total}");
    }

    #[test]
    fn test_part1() {}

    #[test]
    fn test_part2() {}
}
