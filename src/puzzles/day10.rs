use std::fs::read_to_string;

use petgraph::{
    algo::{all_simple_paths, astar},
    graph::NodeIndex,
    Graph,
};
use simple_grid::Grid;

fn parse_input(
    input: &str,
) -> (
    Graph<u32, u32, petgraph::Directed>,
    Vec<NodeIndex>,
    Vec<NodeIndex>,
) {
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
    let mut zero_indices = vec![];
    let mut nine_indices = vec![];
    let mut graph = Graph::new();
    for cell in grid.cell_iter() {
        let node = graph.add_node(*cell);
        if *cell == 0 {
            zero_indices.push(node);
        }
        if *cell == 9 {
            nine_indices.push(node);
        }
    }
    for (i, (x, y)) in grid
        .indices()
        .map(|idx| (idx.column(), idx.row()))
        .enumerate()
    {
        let node = NodeIndex::from(i as u32);
        let weight = *graph.node_weight(node).unwrap();
        grid.cardinal_neighbor_indices_of((x, y)).for_each(|i| {
            let nbr_node_ix = NodeIndex::from((i.row() * grid.width() + i.column()) as u32);
            let cell = *grid.get(i).unwrap();
            if cell > weight && cell - weight == 1 {
                graph.add_edge(node, nbr_node_ix, 1);
            }
        });
    }
    (graph, zero_indices, nine_indices)
}

fn part1(graph: &Graph<u32, u32>, zeroes: &[NodeIndex], nines: &[NodeIndex]) -> usize {
    zeroes
        .iter()
        .flat_map(|start| {
            nines.iter().filter_map(|finish| {
                astar(&graph, *start, |f| f == *finish, |e| *e.weight(), |_| 9)
            })
        })
        .count()
}

fn part2(graph: &Graph<u32, u32>, zeroes: &[NodeIndex], nines: &[NodeIndex]) -> usize {
    zeroes
        .iter()
        .map(|start| {
            nines
                .iter()
                .map(|finish| {
                    all_simple_paths::<Vec<_>, _>(&graph, *start, *finish, 0, None).count()
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn solve() {
    let input = read_to_string("inputs/day10.txt").expect("error opening file");
    let (graph, zeroes, nines) = parse_input(input.strip_suffix("\n").unwrap());
    println!("Part 1: {}", part1(&graph, &zeroes, &nines));
    println!("Part 2: {}", part2(&graph, &zeroes, &nines));
}

#[cfg(test)]
mod test {
    use petgraph::algo::astar;

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
        let char_count = TEST_INPUT.lines().flat_map(|l| l.chars()).count();
        let (graph, _, _) = parse_input(TEST_INPUT);
        assert_eq!(graph.node_count(), char_count);
    }

    #[test]
    fn test_astar() {
        let (graph, zeroes, nines) = parse_input(TEST_INPUT);
        assert!(astar(&graph, zeroes[0], |f| f == nines[0], |e| *e.weight(), |_| 9).is_some());
        assert!(astar(&graph, zeroes[0], |f| f == nines[1], |e| *e.weight(), |_| 9).is_none());
    }

    #[test]
    fn test_part1() {
        let (graph, zeroes, nines) = parse_input(TEST_INPUT);
        assert_eq!(36, part1(&graph, &zeroes, &nines));
    }

    #[test]
    fn test_part2() {
        let (graph, zeroes, nines) = parse_input(TEST_INPUT);
        assert_eq!(81, part2(&graph, &zeroes, &nines));
    }
}
