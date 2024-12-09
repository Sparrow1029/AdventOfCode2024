use std::fs::read_to_string;

fn parse_input(input: &str) -> Vec<u32> {
    input
        .strip_suffix("\n")
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).expect("should be a digit"))
        .collect()
}

fn expand(disk_map: &[u32]) -> Vec<u32> {
    let mut new_vec: Vec<u32> = vec![0; disk_map.iter().map(|d| *d as usize).sum()];
    let mut disk_ptr = 0;
    for (cur_id, (idx, file_size)) in disk_map.iter().enumerate().step_by(2).enumerate() {
        let blocks = *file_size as usize;
        let new_blocks = vec![cur_id as u32; blocks];

        let start_free = disk_ptr + blocks;
        let amt_free = if idx == disk_map.len() - 1 {
            0
        } else {
            disk_map[idx + 1] as usize
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

fn compress(disk: &mut [u32], start: usize) {
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

fn compress_v2(disk_map: &[u32]) -> usize {
    let mut right_idx = disk_map.len() - 1;
    right_idx
}

fn checksum(compressed_disk: &[u32]) -> usize {
    compressed_disk
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, val)| acc + idx * (*val) as usize)
}

pub fn solve() {
    let input = read_to_string("inputs/day09.txt").expect("read file");
    let disk_map = parse_input(&input);
    let mut disk = expand(&disk_map);
    let start = disk_map[0] as usize;
    compress(&mut disk, start);
    let first_zero = disk[1..].iter().position(|v| *v == 0).unwrap();
    println!("Part 1: {}", checksum(&disk));
}

#[cfg(test)]
mod test {
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
                    '0'..='9' => c.to_digit(10).unwrap(),
                    '.' => 0,
                    _ => panic!(),
                })
                .collect::<Vec<u32>>()
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
                    '0'..='9' => c.to_digit(10).unwrap(),
                    '.' => 0,
                    _ => panic!(),
                })
                .collect::<Vec<u32>>()
        );
        assert_eq!(checksum(&expanded), 1928);
    }
}
