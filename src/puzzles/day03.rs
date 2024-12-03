use regex::Regex;

use crate::shared::util::read_lines;

fn parse_mul(input: &[String]) -> (u32, u32) {
    // Lazy, but rejoin after having removed `\n`ewlines.
    let joined = input.concat();
    let re = Regex::new(r"mul\((\d+,\d+)\)|(do\(\))|(don't\(\))").expect("invalid regex");

    // part1
    let mut total1 = 0u32;
    // part2 -- adds `do()`s and `don't()`s
    let mut total2 = 0u32;
    let mut enabled = 1u32;

    re.captures_iter(&joined).for_each(|c| {
        let (_, [m]) = c.extract();
        match m {
            "do()" => enabled = 1,
            "don't()" => enabled = 0,
            _ => {
                let product: u32 = m.split(",").map(|s| s.parse::<u32>().unwrap()).product();
                total1 += product;
                total2 += product * enabled;
            }
        }
    });
    (total1, total2)
}

pub fn solve() {
    let input = read_lines("inputs/day03.txt");
    let (part1_res, part2_res) = parse_mul(&input);
    println!("Part 1: {}", part1_res);
    println!("Part 2: {}", part2_res);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        let test_input = vec![
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string(),
        ];
        let (p1, p2) = parse_mul(&test_input);
        eprintln!("P1: {p1}, P2: {p2}");
        assert_eq!(161, p1);
        assert_eq!(48, p2);
    }
}
