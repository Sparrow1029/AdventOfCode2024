use std::fs::read_to_string;

use crate::shared::util::wait_millis;

fn parse_input(input: &str) -> Vec<usize> {
    input
        .strip_suffix("\n")
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).expect("should be a digit") as usize)
        .collect()
}

fn expand(disk_map: &[usize]) -> Vec<usize> {
    let mut new_vec: Vec<usize> = vec![0; disk_map.iter().sum()];
    let mut disk_ptr = 0;
    for (cur_id, (idx, blocks)) in disk_map.iter().enumerate().step_by(2).enumerate() {
        let new_blocks = vec![cur_id; *blocks];

        let start_free = disk_ptr + blocks;
        let amt_free = if idx == disk_map.len() - 1 {
            0
        } else {
            disk_map[idx + 1]
        };
        let free_space = vec![0; amt_free];

        new_vec.splice(disk_ptr..start_free, new_blocks);
        disk_ptr += amt_free + blocks;

        if disk_ptr == new_vec.len() - 1 {
            break;
        }
        new_vec.splice(start_free..start_free + amt_free, free_space);
    }
    new_vec
}

fn compress(disk: &mut [usize], start: usize) {
    let mut left_idx = start;
    let mut right_idx = disk.len() - 1;
    while !disk[left_idx..].iter().all(|d| *d == 0) {
        if disk[left_idx] == 0 {
            while disk[right_idx] == 0 {
                right_idx -= 1;
            }
            disk.swap(left_idx, right_idx);
        }
        left_idx += 1;
    }
}

/// Take each file block from the end, check if it can fit in any free space starting from the
/// left. If so, swap out its location with all zeros and place it in the free space.
/// Increment/decrement a bunch of counters to keep track of which slice we are currently operating
/// on.
fn compress_v2(disk: &mut Vec<usize>, disk_map: &[usize]) -> usize {
    let start = disk_map[0];
    let mut disk_ptr = disk.len() - disk_map[disk_map.len() - 1];
    let mut map_ptr = disk_map.len() - 1;
    let mut zero_pos = start;
    'outer: loop {
        let block_size = disk_map[map_ptr];
        let cur_block = disk[disk_ptr..disk_ptr + block_size].to_vec();
        let check_slice = disk[zero_pos..disk_ptr].to_vec();
        let windows = check_slice.windows(block_size).enumerate();
        let to_swap = vec![0; block_size];

        for (idx, slice) in windows {
            if slice.iter().all(|d| *d == 0) {
                let cur_idx = idx + zero_pos;
                disk.splice(cur_idx..cur_idx + block_size, cur_block.clone());
                disk.splice(disk_ptr..disk_ptr + block_size, to_swap.clone());
                break;
            }
        }
        // increment all the things
        map_ptr -= 2;
        disk_ptr -= disk_map[map_ptr] + disk_map[map_ptr + 1];
        zero_pos = disk[start..].iter().position(|d| *d == 0).unwrap() + start;
        if zero_pos >= disk_ptr {
            break 'outer;
        }
    }
    checksum(disk)
}

fn checksum(compressed_disk: &[usize]) -> usize {
    compressed_disk
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, val)| acc + idx * (*val))
}

pub fn solve() {
    let input = read_to_string("inputs/day09.txt").expect("read file");
    let disk_map = parse_input(&input);
    let mut disk = expand(&disk_map);
    let start = disk_map[0];
    compress(&mut disk.clone(), start);
    println!("Part 1: {}", checksum(&disk));
    println!("Part 2: {}", compress_v2(&mut disk, &disk_map));
}

#[cfg(test)]
mod test {
    use crate::shared::util::wait_millis;

    use super::*;

    const TEST_INPUT: &str = "2333133121414131402\n";

    #[test]
    fn test_expand() {
        let disk_map = parse_input(TEST_INPUT);
        let expanded = expand(&disk_map);
        assert_eq!(
            expanded,
            "00...111...2...333.44.5555.6666.777.888899"
                .chars()
                .map(|c| match c {
                    '0'..='9' => c.to_digit(10).unwrap() as usize,
                    '.' => 0,
                    _ => panic!(),
                })
                .collect::<Vec<usize>>()
        )
    }

    #[test]
    fn test_compress() {
        let disk_map = parse_input(TEST_INPUT);
        let mut expanded = expand(&disk_map);
        compress(&mut expanded, disk_map[0] as usize);
        assert_eq!(
            expanded,
            "0099811188827773336446555566.............."
                .chars()
                .map(|c| match c {
                    '0'..='9' => c.to_digit(10).unwrap() as usize,
                    '.' => 0,
                    _ => panic!(),
                })
                .collect::<Vec<usize>>()
        );
        assert_eq!(checksum(&expanded), 1928);
    }

    #[test]
    fn test_compress_v2() {
        let disk_map = parse_input(TEST_INPUT);
        let mut disk = expand(&disk_map);
        println!("DISK: {disk:?}");
        wait_millis(1000);
        compress_v2(&mut disk, &disk_map);
        assert_eq!(
            disk,
            "00992111777.44.333....5555.6666.....8888.."
                .chars()
                .map(|c| match c {
                    '0'..='9' => c.to_digit(10).unwrap() as usize,
                    '.' => 0,
                    _ => panic!(),
                })
                .collect::<Vec<usize>>()
        );
        assert_eq!(checksum(&disk), 2858);
    }
}
