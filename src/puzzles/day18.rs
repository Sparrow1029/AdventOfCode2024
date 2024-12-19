use std::fs::read_to_string;

use crate::shared::graph_util::{get_nbr_indices, get_node_idx};
use crate::shared::util::clear_screen;
use itertools::Itertools;
use petgraph::algo::astar;
use petgraph::prelude::{EdgeRef, Graph, NodeIndex, Undirected};

fn parse_input(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|line| {
            let split = line.split_once(',').unwrap();
            (
                split.0.parse().expect("not digit"),
                split.1.parse().expect("not digit"),
            )
        })
        .collect_vec()
}

fn graph_from_bytes(bytes: &[(u32, u32)], width: u32, height: u32) -> Graph<u32, u32, Undirected> {
    let mut edges = vec![];
    for y in 0..height {
        for x in 0..width {
            if !bytes.contains(&(x, y)) {
                let node_idx = y * width + x;
                let neighbors = get_nbr_indices(node_idx, width as usize, height as usize);
                for (_, nbr) in neighbors {
                    let (nx, ny) = (nbr.index() as u32 % width, nbr.index() as u32 / width);
                    if !bytes.contains(&(nx, ny)) {
                        edges.push((node_idx, nbr.index() as u32, 1u32));
                    }
                }
            }
        }
    }
    Graph::from_edges(&edges)
}

fn display_graph(
    graph: &Graph<u32, u32, Undirected>,
    path: &[NodeIndex],
    bytes: &[(u32, u32)],
    width: u32,
    height: u32,
) -> String {
    (0u32..height)
        .map(|y| {
            (0u32..width)
                .flat_map(|x| {
                    let node_idx = get_node_idx(x, y, width);
                    if path.contains(&node_idx) {
                        Some('O')
                    } else if bytes.contains(&(x, y)) {
                        Some('#')
                    } else {
                        Some('.')
                    }
                })
                .join(" ")
        })
        .join("\n")
}

fn remove_edges_for_xy(graph: &mut Graph<u32, u32, Undirected>, x: u32, y: u32, width: u32) {
    let node_idx = get_node_idx(x, y, width);
    let edges = graph.edges(node_idx).map(|eref| eref.id()).collect_vec();
    for edge in edges {
        _ = graph.remove_edge(edge);
    }
}

pub fn solve() {
    let input = read_to_string("inputs/day18.txt").expect("file read error");
    let bytes = parse_input(&input);
    println!("bytes_len: {}", bytes.len());
    let (width, height) = (71, 71);
    let (start, end): (NodeIndex, NodeIndex) = (0u32.into(), ((width * height) - 1).into());
    let mut graph = graph_from_bytes(&bytes[..1024], width, height);
    let (pt1_cost, pt1_path) = astar(&graph, start, |f| f == end, |e| *e.weight(), |_| 0).unwrap();
    log::debug!(
        "{}",
        display_graph(&graph, &pt1_path, &bytes, width, height)
    );
    println!("Part 1: {}", pt1_cost);
    // Part 2
    for (cur_byte, &(x, y)) in bytes[1025..].iter().enumerate() {
        remove_edges_for_xy(&mut graph, x, y, width);
        if let Some((_, path)) = astar(&graph, start, |f| f == end, |e| *e.weight(), |_| 0) {
            if log::log_enabled!(log::Level::Debug) {
                clear_screen();
                log::debug!(
                    "{}",
                    display_graph(&graph, &path, &bytes[..cur_byte + 1025], width, height)
                );
            }
            continue;
        } else {
            println!("Part 2: ({x}, {y})",);
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use petgraph::adj::NodeIndex;

    use super::*;
    use crate::shared::util::test_setup;

    const TEST_INPUT: &str = "5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n2,6\n5,1\n1,2\n5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n6,1\n1,0\n0,5\n1,6\n2,0";

    #[test]
    fn test_parse() {
        test_setup();
        let bytes = parse_input(TEST_INPUT);
        assert_eq!(bytes.len(), 25);
    }

    #[test]
    fn test_pt1() {
        test_setup();
        let bytes = parse_input(TEST_INPUT);
        let (width, height) = (7, 7);
        let graph = graph_from_bytes(&bytes[..12], width, height);
        println!("{graph:?}");
        let (cost, path) = astar(
            &graph,
            NodeIndex::from(0u32),
            |f| f == NodeIndex::from(48),
            |e| *e.weight(),
            |_| 0,
        )
        .unwrap();
        log::debug!("\n{}", display_graph(&graph, &path, &bytes, width, height));
        assert_eq!(cost, 22);
    }
}
