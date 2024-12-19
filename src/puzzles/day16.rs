use core::panic;

use crate::shared::graph_util::get_nbr_indices;
use itertools::Itertools;
use petgraph::{algo::astar, graph::NodeIndex, Graph, Undirected};

#[derive(Debug)]
struct Maze {
    start: NodeIndex,
    end: NodeIndex,
    width: usize,
    height: usize,
    graph: Graph<char, u32, Undirected>,
}

impl Maze {
    fn display_path(&self, path: &[NodeIndex]) {
        println!(
            "{}",
            (0u32..self.height as u32)
                .map(|y| {
                    (0u32..self.width as u32)
                        .flat_map(|x| {
                            let node_idx = NodeIndex::from(y * self.width as u32 + x);
                            if path.contains(&node_idx) {
                                Some('o')
                            } else {
                                Some(self.graph[node_idx])
                            }
                        })
                        .collect::<String>()
                })
                .join("\n")
        )
    }
}

fn parse_input(input: &str) -> Maze {
    let width = input.lines().peekable().peek().unwrap().chars().count();
    let height = input.lines().count();
    let mut graph: Graph<char, u32, Undirected> = Graph::new_undirected();
    let mut flat_vec = input.lines().flat_map(|line| line.chars()).collect_vec();
    let start = match flat_vec.iter().position(|&c| c == 'S') {
        Some(pos) => NodeIndex::from(pos as u32),
        None => panic!("no start"),
    };
    let end = match flat_vec.iter().position(|&c| c == 'E') {
        Some(pos) => NodeIndex::from(pos as u32),
        None => panic!("no end"),
    };
    flat_vec[start.index()] = '.';
    flat_vec[end.index()] = '.';

    for c in &flat_vec {
        graph.add_node(*c);
    }
    for idx in (0u32..flat_vec.len() as u32).map(NodeIndex::from) {
        let cur_node = graph[idx];
        for (cost, nbr) in get_nbr_indices(idx, width, height) {
            match (cur_node, graph[nbr]) {
                ('#', '.') | ('.', '#') | ('#', '#') => {}
                ('.', '.') => {
                    graph.add_edge(idx, nbr, cost);
                }
                _ => panic!("invalid characters: {cur_node}, {}", graph[nbr]),
            };
        }
    }
    Maze {
        start,
        end,
        width,
        height,
        graph,
    }
}

fn part1_astar(maze: &Maze) -> Option<(u32, Vec<NodeIndex>)> {
    astar(
        &maze.graph,
        maze.start,
        |f| f == maze.end,
        |e| *e.weight(),
        |_| (maze.width * maze.height) as u32,
    )
}

pub fn solve() {
    println!("Part 1: TODO!");
    println!("Part 2: TODO!");
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::shared::util::test_setup;

    const TEST_INPUT: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const TEST_INPUT2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_parse() {
        test_setup();
        let maze = parse_input(TEST_INPUT);
        assert_eq!(maze.height, 15);
        assert_eq!(maze.width, 15);
    }

    #[test]
    fn test_part1() {
        test_setup();
        let maze = parse_input(TEST_INPUT);
        let (dist, path) = part1_astar(&maze).unwrap();
        println!("dist: {dist}, path: {path:?}");
        maze.display_path(&path);
        let maze2 = parse_input(TEST_INPUT);
        let (dist2, path2) = part1_astar(&maze2).unwrap();
        maze2.display_path(&path2);
        println!("dist: {dist2}, path: {path2:?}");
    }

    #[test]
    #[ignore = "wip"]
    fn test_part2() {
        test_setup();
    }
}
