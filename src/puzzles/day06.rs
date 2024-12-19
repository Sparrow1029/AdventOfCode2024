use std::{collections::HashSet, fs::read_to_string};

use crate::shared::grid2d::Direction;
use simple_grid::{Grid, GridIndex};

type Idx = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Obstacle,
    Guard,
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

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Empty => '.',
            Tile::Obstacle => '#',
            Tile::Guard => '@',
        };
        write!(f, "{}", c)
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
                    return direction.get_next_idx(cur_pos);
                }
                _ => panic!("oh no"),
            }
        } else {
            return None;
        }
    }
    None
}

fn walk(board: &mut Grid<Tile>, start: Idx) -> HashSet<Idx> {
    let mut guard_pos = start;
    let mut cur_dir = Direction::Up;
    let mut visited = HashSet::from([guard_pos]);
    while let Some(next_idx) = do_move(guard_pos, &mut cur_dir, board) {
        // wait_millis(100);
        // clear_screen();
        // let old = board.replace_cell(guard_pos, Tile::Guard);
        // println!("{}", board.to_pretty_string());
        // board.replace_cell(guard_pos, old);
        // println!("next_idx: {next_idx:?}");
        guard_pos = next_idx;
        visited.insert(guard_pos);
    }
    visited
}

#[allow(unused)]
fn detect_loop_walk(board: &Grid<Tile>, start: Idx, wall: Idx) -> bool {
    let mut board_clone = board.clone();
    board_clone.replace_cell(wall, Tile::Obstacle);
    let mut guard_pos = start;
    let mut cur_dir = Direction::Up;
    let mut visited = HashSet::from([(start, cur_dir)]);
    loop {
        // wait_millis(10);
        // clear_screen();
        // let old = board_clone.replace_cell(guard_pos, Tile::Guard);
        // println!("{}", board_clone.to_pretty_string());
        // board_clone.replace_cell(guard_pos, old);
        if let Some(pos) = cur_dir.get_next_idx(guard_pos) {
            match board_clone.get(pos) {
                Some(Tile::Obstacle) => {
                    // Check if we've faced this exact obstacle before
                    if visited.contains(&(pos, cur_dir)) {
                        break true;
                    }
                    // only need to store collisions with obstacles
                    visited.insert((pos, cur_dir));
                    cur_dir = cur_dir.turn_right();
                }
                Some(Tile::Empty) => {
                    guard_pos = pos;
                    visited.insert((pos, cur_dir));
                }
                Some(Tile::Guard) => {
                    panic!("no other guards on the board");
                }
                // Guard has left area
                None => {
                    break false;
                }
            }
        } else {
            break false;
        }
    }
}

#[allow(unused)]
fn part2(board: &Grid<Tile>, start: Idx) -> usize {
    println!("Board: {}, {}", board.width(), board.height());
    board
        .indices()
        .filter(|grid_index| {
            let pos = (grid_index.column(), grid_index.row());
            // println!("trying {pos:?}");
            pos.0!= start.0 && pos.1 != start.1 // don't check original guard start
            && board.get(pos).unwrap() == &Tile::Empty
            && detect_loop_walk(board, start, pos)
        })
        .count()
}

pub fn solve() {
    let input = read_to_string("inputs/day06.txt").expect("file read error");
    let (mut board, start) = parse(&input);
    let visited = walk(&mut board, start);
    println!("Part 1: {}", visited.len());
    // println!("Part 2: {}", part2(&board, start));
    println!("Part 2: TOO_SLOW");
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
        let (mut board, start) = parse(TEST_INPUT);
        let visited = walk(&mut board, start);
        assert_eq!(41, visited.len());
    }

    #[test]
    fn test_detect_loops() {
        let (board, start) = parse(TEST_INPUT);
        let loops = part2(&board, start);
        assert_eq!(6, loops);
    }
}
