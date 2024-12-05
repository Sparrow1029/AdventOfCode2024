use grid::Grid;

use crate::shared::{
    grid2d::{iter_diag_nesw, iter_diag_nwse, Point},
    util::read_lines,
};

fn parse_grid(input: &[String]) -> Grid<u8> {
    let cols = input.first().unwrap().len();
    Grid::from_vec(
        input
            .iter()
            .flat_map(|row| row.chars().map(|c| c as u8).collect::<Vec<u8>>())
            .collect(),
        cols,
    )
}

fn part1(grid: &Grid<u8>) -> usize {
    let mut xmas_count = 0;
    let rows = grid
        .iter_rows()
        .map(|d| String::from_utf8(d.copied().collect()).unwrap());
    let cols = grid
        .iter_cols()
        .map(|d| String::from_utf8(d.copied().collect()).unwrap());
    for diag in iter_diag_nesw(grid)
        .chain(iter_diag_nwse(grid))
        .filter_map(|d| {
            if d.len() >= 4 {
                Some(String::from_utf8(d.clone()).unwrap())
            } else {
                None
            }
        })
        .chain(rows)
        .chain(cols)
    {
        xmas_count += diag.matches("XMAS").count() + diag.matches("SAMX").count()
    }
    xmas_count
}

fn part2(grid: &Grid<u8>) -> usize {
    let mut xmas_count = 0;
    let valid = [
        [b'M', b'M', b'S', b'S'],
        [b'M', b'S', b'S', b'M'],
        [b'S', b'M', b'M', b'S'],
        [b'S', b'S', b'M', b'M'],
    ];
    for x in 1..grid.cols() - 1 {
        for y in 1..grid.rows() - 1 {
            if grid.get(y, x) == Some(&b'A')
                && valid.contains(
                    &(Point::new(x as isize, y as isize)
                        .diagonal_neighbors(grid)
                        .map(|i| i.unwrap_or(0))),
                )
            {
                // println!(
                //     "{:?}, {:?}",
                //     *grid.get(y, x).unwrap_or(&b'Z') as char,
                //     Point::new(x as isize, y as isize)
                //         .diagonal_neighbors(grid)
                //         .map(|c| c.unwrap_or(b'.') as char)
                // );
                xmas_count += 1;
            }
        }
    }
    xmas_count
}

pub fn solve() {
    let input = read_lines("inputs/day04.txt");
    let grid = parse_grid(&input);
    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part_1() {
        let input = TEST_INPUT.lines().map(str::to_string).collect::<Vec<_>>();
        let grid = parse_grid(&input);
        let result = part1(&grid);
        assert_eq!(18, result)
    }

    #[test]
    fn test_part_2() {
        let input = TEST_INPUT.lines().map(str::to_string).collect::<Vec<_>>();
        let grid = parse_grid(&input);
        let result = part2(&grid);
        assert_eq!(9, result)
    }
}
