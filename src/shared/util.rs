use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .expect("error reading line")
        .lines()
        .map(|l| l.to_owned())
        .collect()
}
