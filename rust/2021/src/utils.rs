use std::env;
use std::fmt::Display;
use std::fs;
use std::time::Instant;

pub fn read_file(folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd
        .join("src")
        .join(folder)
        .join(format!("day{:02}.txt", day));
    println!("{:?}", filepath);
    let f = fs::read_to_string(filepath);
    f.expect("Could not open input file, wrong path!")
}

static ANSI_ITALIC: &str = "\x1b[3m";
static ANSI_RESET: &str = "\x1b[0m";

pub fn print_result<T: Display>(func: impl FnOnce(&str) -> T, input: &str) {
    let timer = Instant::now();
    let result = func(input);
    let time = timer.elapsed();
    println!(
        "Result: {} {}(elapsed time: {:.2?}){}",
        result, ANSI_ITALIC, time, ANSI_RESET
    );
}
