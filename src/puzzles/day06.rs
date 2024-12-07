use std::{collections::HashSet, fs::read_to_string};

use simple_grid::{Grid, GridIndex};

type Idx = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Obstacle,
    Guard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn get_next_idx(&self, ix: (usize, usize)) -> Option<Idx> {
        let idx = GridIndex::from(ix);
        match self {
            Direction::Up => idx.up(),
            Direction::Right => idx.right(),
            Direction::Down => idx.down(),
            Direction::Left => idx.left(),
        }
        .map(|i| (i.column(), i.row()))
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Obstacle,
            '^' => Self::Guard,
            _ => panic!("unrecognized character"),
        }
    }
}

fn parse(input: &str) -> (Grid<Tile>, Idx) {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut grid = Grid::new(
        lines[0].len(),
        lines.len(),
        lines
            .iter()
            .flat_map(|l| l.chars().map(Tile::from))
            .collect(),
    );
    let start = grid.position(|&t| t == Tile::Guard).unwrap();
    grid.replace_cell(start, Tile::Empty);
    (grid, (start.column(), start.row()))
}

fn do_move(cur_pos: Idx, direction: &mut Direction, board: &Grid<Tile>) -> Option<Idx> {
    if let Some(next_idx) = direction.get_next_idx(cur_pos) {
        if let Some(tile) = board.get(GridIndex::from(next_idx)) {
            match tile {
                Tile::Empty => {
                    return Some(next_idx);
                }
                Tile::Obstacle => {
                    *direction = direction.turn_right();
                    direction.get_next_idx(next_idx);
                }
                _ => panic!("oh no"),
            }
        } else {
            return None;
        }
    }
    None
}

fn walk(board: &Grid<Tile>, start: Idx) -> HashSet<(Idx, Direction)> {
    let mut guard_pos = start;
    let mut cur_dir = Direction::Up;
    let mut visited = HashSet::from([(guard_pos, cur_dir)]);
    while let Some(next_idx) = do_move(guard_pos, &mut cur_dir, board) {
        guard_pos = next_idx;
        visited.insert((guard_pos, cur_dir));
    }
    visited
}

fn detect_loop(board: &Grid<Tile>, start: Idx) -> bool {
    let mut guard_pos = start;
    let mut cur_dir = Direction::Up;
    let mut visited = HashSet::from([(guard_pos, cur_dir)]);
    loop {
        if let Some(pos) = cur_dir.get_next_idx(guard_pos) {
            match board.get(pos) {
                Some(Tile::Obstacle) => {
                    // Check if we've faced this exact obstacle before
                    if visited.contains(&(pos, cur_dir)) {
                        break true;
                    }
                    visited.insert((pos, cur_dir));
                    cur_dir = cur_dir.turn_right();
                    guard_pos = pos;
                }
                Some(Tile::Empty) => {
                    guard_pos = pos;
                }
                Some(Tile::Guard) => {
                    panic!("no other guards on the board");
                }
                // Guard has left area
                None => {
                    break false;
                }
            }
        }
    }
}

pub fn solve() {
    let input = read_to_string("inputs/day06.txt").expect("file read error");
    let (board, start) = parse(&input);
    println!("Part 1: {}", walk(&board, start).len());
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_parse() {
        let (board, _) = parse(TEST_INPUT);
        assert_eq!(10, board.width());
    }

    #[test]
    fn test_walk() {
        let (board, start) = parse(TEST_INPUT);
        let visited = walk(&board, start);
        assert_eq!(41, visited.len());
    }
}
