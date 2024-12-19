use std::collections::VecDeque;

use simple_grid::{Grid, GridIndex};

use crate::shared::grid2d::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Player,
    Box,
    Wall,
}

fn parse_input(input: &str) -> (Grid<Tile>, VecDeque<Direction>) {
    let split = input.split("\n\n").collect::<Vec<_>>();
    let (map, moves) = (split[0], split[1]);
    let width = map.lines().next().unwrap().chars().count();
    let height = map.lines().count();
    (
        Grid::new(
            width,
            height,
            map.lines()
                .flat_map(|line| {
                    line.chars().map(|c| match c {
                        '.' => Tile::Empty,
                        '@' => Tile::Player,
                        'O' => Tile::Box,
                        '#' => Tile::Wall,
                        _ => panic!("invalid char"),
                    })
                })
                .collect(),
        ),
        moves
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    '^' => Direction::Up,
                    '>' => Direction::Right,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    _ => panic!("invalid char"),
                })
            })
            .collect(),
    )
}

fn find_player(grid: &Grid<Tile>) -> GridIndex {
    grid.cells_with_indices_iter()
        .find(|(_, &cell)| cell == Tile::Player)
        .unwrap()
        .0
}

fn tick(grid: &mut Grid<Tile>, cur_move: Direction) {
    let player_pos = find_player(&grid);
    let cur_tile = Some(player_pos);
    let mut cur_stack = Vec::with_capacity(grid.width() * grid.height());
    cur_stack.push(player_pos);
    match cur_move {
        Direction::Up => {}
        Direction::Right => {}
        Direction::Down => {}
        Direction::Left => {
            let row: Vec<Tile> = grid
                .row_iter(player_pos.row())
                .enumerate()
                .filter_map(|(x, &tile)| (x <= player_pos.column()).then_some(tile))
                .collect();
            log::debug!("{row:?}");
        }
    }
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
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_parse() {
        test_setup();
        let (grid, moves) = parse_input(TEST_INPUT);
        assert_eq!(find_player(&grid), (4, 4).into());
        assert_eq!(moves[3], Direction::Right);
    }

    #[test]
    fn test_part1() {
        test_setup();
        let (mut grid, mut moves) = parse_input(TEST_INPUT);
        tick(&mut grid, moves.pop_front().unwrap());
    }

    #[test]
    #[ignore = "wip"]
    fn test_part2() {
        test_setup();
    }
}
