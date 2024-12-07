use std::fs::read_to_string;

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .expect("error reading line")
        .lines()
        .map(|l| l.to_owned())
        .collect()
}

pub fn sleep(s: u64) {
    let time = std::time::Duration::from_millis(s);
    std::thread::sleep(time);
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    print!("{esc}[2J{esc}[1;1H", esc = 27u8 as char);
}
