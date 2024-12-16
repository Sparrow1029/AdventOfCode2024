use std::fs::read_to_string;
use std::io::Write;

use std::sync::Once;

static INIT_TEST_LOGGER: Once = Once::new();

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .expect("error reading line")
        .lines()
        .map(|l| l.to_owned())
        .collect()
}

pub fn wait_millis(s: u64) {
    let time = std::time::Duration::from_millis(s);
    std::thread::sleep(time);
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    print!("{esc}[2J{esc}[1;1H", esc = 27u8 as char);
}

pub fn test_setup() {
    INIT_TEST_LOGGER.call_once(|| {
        env_logger::builder()
            .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
            .init()
    });
}
