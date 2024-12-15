use crate::shared::point::Point;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn parse_input(input: &str) -> (HashMap<char, Vec<Point>>, Point) {
    let max_x = input.trim().lines().peekable().peek().unwrap().len() - 1;
    let max_y = input.trim().lines().count() - 1;
    let mut map: HashMap<char, Vec<Point>> = HashMap::new();
    input.trim().lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| match c {
            '.' => {}
            _ => {
                let pt = Point::new(x as i32, y as i32);
                map.entry(c).and_modify(|v| v.push(pt)).or_insert(vec![pt]);
            }
        })
    });
    (map, (max_x as i32, max_y as i32).into())
}

fn get_anodes(pt1: Point, pt2: Point, max: Point) -> HashSet<Point> {
    let (dx, dy) = (pt1.x - pt2.x, pt2.y - pt1.y);
    let (new_pt1, new_pt2) = if dy == 0 {
        (Point::new(pt1.x - dx, pt1.y), Point::new(pt2.x + dx, pt2.y))
    } else if dx == 0 {
        (Point::new(pt1.x, pt1.y - dy), Point::new(pt2.x, pt2.y + dy))
    } else {
        (
            Point::new(pt1.x + dx, pt1.y - dy),
            Point::new(pt2.x - dx, pt2.y + dy),
        )
    };
    let set = [new_pt1, new_pt2]
        .iter()
        .filter_map(|&p| p.in_bounds((0, 0).into(), max).then_some(p))
        .collect();
    set
}

fn part1(map: &HashMap<char, Vec<Point>>, max: &Point) -> usize {
    let mut anodes = HashSet::new();
    for (_, v) in map.iter() {
        for i in 0..v.len() - 1 {
            for j in i + 1..v.len() {
                anodes.extend(get_anodes(v[i], v[j], *max))
            }
        }
    }
    anodes.len()
}

pub fn solve() {
    let input = read_to_string("inputs/day08.txt").expect("file read err");
    let (map, max) = parse_input(&input);
    println!("Part 1: {}", part1(&map, &max));
    println!("Part 2: TODO!");
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_parse() {
        let (map, _) = parse_input(TEST_INPUT);
        assert_eq!(map[&'0'].len(), 4);
        assert_eq!(map[&'A'].len(), 3);
    }

    #[test]
    fn test_get_anodes() {
        let (map, max) = parse_input(TEST_INPUT);
        let slice = &map[&'0'][1..3];
        let anodes = get_anodes(slice[0], slice[1], max);
        assert!(anodes.contains(&Point::new(9, 4)));
        assert!(anodes.contains(&Point::new(3, 1)));
    }

    #[test]
    fn test_part1() {
        let (map, max) = parse_input(TEST_INPUT);
        let res = part1(&map, &max);
        assert_eq!(res, 14);
    }

    #[test]
    #[ignore = "wip"]
    fn test_part2() {}
}
