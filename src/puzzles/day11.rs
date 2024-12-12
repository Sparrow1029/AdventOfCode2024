use std::collections::HashMap;

/// Create HashMap of present stone values
fn parse_input(input: &str) -> HashMap<u64, usize> {
    input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        })
}

fn split_stone(stone: u64, len: u32) -> (u64, u64) {
    let divisor: u64 = 10i32.pow(len / 2) as u64;
    let left = stone / divisor;
    let right = stone % divisor;
    (left, right)
}

fn blink(stones: &mut HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut new_stones = HashMap::new();
    stones.iter_mut().for_each(|(&stone, count)| {
        if stone == 0 {
            *new_stones.entry(1).or_insert(0) += *count;
        } else {
            let num_digits = stone.checked_ilog10().unwrap_or(0) + 1;
            if num_digits % 2 == 0 {
                let (l, r) = split_stone(stone, num_digits);
                *new_stones.entry(l).or_insert(0) += *count;
                *new_stones.entry(r).or_insert(0) += *count;
            } else {
                *new_stones.entry(stone * 2024).or_insert(0) += *count;
            }
        }
    });
    new_stones
}

pub fn solve() {
    let mut stones = parse_input("5178527 8525 22 376299 3 69312 0 275");
    for _ in 0..25 {
        stones = blink(&mut stones);
    }
    println!("Part 1: {}", stones.values().sum::<usize>());
    for _ in 0..50 {
        stones = blink(&mut stones);
    }
    println!("Part 2: {}", stones.values().sum::<usize>());
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_25_blinks() {
        let mut stones = parse_input(TEST_INPUT);
        for _ in 0..25 {
            stones = blink(&mut stones);
        }
        assert_eq!(stones.values().sum::<usize>(), 55312);
    }
}
