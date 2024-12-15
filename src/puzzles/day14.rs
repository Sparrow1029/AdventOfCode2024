use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap, env, fs::read_to_string};

use crate::shared::{grid2d::FlatGrid, point::Point};
use regex::Regex;

const BOUNDS: Point = Point { x: 101, y: 103 };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Robot {
    point: Point,
    vector: Point,
}

impl std::fmt::Display for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pos: {:?}, vec: {:?}", self.point, self.vector)
    }
}

impl From<((i32, i32), (i32, i32))> for Robot {
    fn from((point, vector): ((i32, i32), (i32, i32))) -> Self {
        Robot {
            point: point.into(),
            vector: vector.into(),
        }
    }
}

impl From<(Point, Point)> for Robot {
    fn from((point, vector): (Point, Point)) -> Self {
        Robot { point, vector }
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"^p\=(?<x>-?\d+),(?<y>-?\d+) v\=(?<vx>-?\d+),(?<vy>-?\d+)$")
        .expect("invalid regex");
    input
        .lines()
        .filter_map(|line| {
            re.captures(line)
                .map(|caps| caps.extract())
                .map(|(_, [x, y, vx, vy])| {
                    (
                        (x.parse().unwrap(), y.parse().unwrap()),
                        (vx.parse().unwrap(), vy.parse().unwrap()),
                    )
                        .into()
                })
        })
        .collect()
}

fn move_robot(robot: &Robot, bounds: Point, seconds: i32) -> Point {
    (
        (robot.point.x + robot.vector.x * seconds).rem_euclid(bounds.x),
        (robot.point.y + robot.vector.y * seconds).rem_euclid(bounds.y),
    )
        .into()
}

fn move_all_robots(robots: &[Robot], bounds: Point, seconds: i32) -> HashMap<Point, usize> {
    let mut pos_counts: HashMap<Point, usize> = HashMap::new();
    for robot in robots {
        let moved = move_robot(robot, bounds, seconds);
        pos_counts
            .entry(moved)
            .and_modify(|cnt| *cnt += 1)
            .or_insert(1);
    }
    pos_counts
}

fn part1(robots: &[Robot], bounds: Point, seconds: i32) -> usize {
    let mut product = 1;
    let pos_counts = move_all_robots(robots, bounds, seconds);
    let (x_div, y_div) = (bounds.x / 2, bounds.y / 2);
    for (x, x_ord, y, y_ord) in [x_div, y_div]
        .iter()
        .cartesian_product([Ordering::Less, Ordering::Greater])
        .combinations(2)
        .filter(|pair| pair[0].0 != pair[1].0)
        .map(|v| (v[0].0, v[0].1, v[1].0, v[1].1))
    {
        product *= pos_counts
            .iter()
            .filter_map(|(&k, v)| (k.x.cmp(x) == x_ord && k.y.cmp(y) == y_ord).then_some(v))
            .sum::<usize>();
    }
    product
}

fn part2(robots: &[Robot], bounds: Point) -> i32 {
    // Stolen from someone elses answer :'(
    use rayon::prelude::*;
    if let Some(seconds) = (100..(bounds.x * bounds.y))
        .into_par_iter()
        .find_first(|t| {
            let mut grid = vec![0_u128; bounds.y as usize];

            for robot in robots {
                let (x, y) = (
                    (robot.point.x + robot.vector.x * t).rem_euclid(bounds.x) as usize,
                    (robot.point.y + robot.vector.y * t).rem_euclid(bounds.y) as usize,
                );

                grid[y] |= 1 << x;
            }

            // looking to see 2 rows with a continous line of at least 16 robots
            // this will match the lines above and below the christmas tree
            grid.iter()
                .filter(|&row| {
                    (0..bounds.x)
                        .step_by(16)
                        .map(|x| 0xFFFF << x)
                        .any(|mask| row & mask == mask)
                })
                .take(2)
                .count()
                >= 2
        })
    {
        seconds
    } else {
        panic!("no solution")
    }
}

fn flat_grid_from_robots(robots: &[Robot], width: usize, height: usize) -> FlatGrid<char> {
    let mut grid = FlatGrid {
        width,
        height,
        data: vec!['.'; width * height],
    };
    for robot in robots {
        grid.replace_cell(robot.point.x as usize, robot.point.y as usize, '0');
    }
    grid
}

pub fn solve() {
    let input = read_to_string("inputs/day14.txt").expect("file read err");
    let robots = parse_input(&input);
    println!("Part 1: {}", part1(&robots, BOUNDS, 100));
    let easter_egg_seconds = part2(&robots, BOUNDS);
    println!("Part 2: {}", easter_egg_seconds);
    match env::var("DEBUG") {
        Ok(pretty_print) if ["true", "t", "1"].contains(&pretty_print.as_str()) => {
            println!(
                "{}",
                flat_grid_from_robots(
                    &robots
                        .iter()
                        .map(|r| Robot {
                            point: move_robot(r, BOUNDS, easter_egg_seconds),
                            vector: r.vector
                        })
                        .collect_vec(),
                    BOUNDS.x as usize,
                    BOUNDS.y as usize
                )
            );
        }
        _ => {}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_parse() {
        let robots = parse_input(TEST_INPUT);
        assert_eq!(robots.len(), 12);
        assert_eq!(
            robots[0],
            Robot {
                point: (0, 4).into(),
                vector: (3, -3).into()
            }
        );
    }

    #[test]
    fn test_move() {
        assert_eq!(
            move_robot(
                &Robot {
                    point: (2, 4).into(),
                    vector: (2, -3).into(),
                },
                (11, 7).into(),
                5,
            ),
            (1, 3).into()
        );
    }

    #[test]
    fn test_part1() {
        let robots = parse_input(TEST_INPUT);
        let product = part1(&robots, (11, 7).into(), 100);
        println!("product: {product}");
    }
}
