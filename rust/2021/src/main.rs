use std::env;
mod utils;
use aoc::utils::read_file;
mod solutions;
use crate::solutions::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: u8 = args[1].clone().parse().unwrap();
    let input = read_file("input", day);

    match day {
        1 => aoc::solve_day!(day01, &input),
        2 => aoc::solve_day!(day02, &input),
        3 => aoc::solve_day!(day03, &input),
        4 => aoc::solve_day!(day04, &input),
        5 => aoc::solve_day!(day05, &input),
        6 => aoc::solve_day!(day06, &input),
        7 => aoc::solve_day!(day07, &input),
        8 => aoc::solve_day!(day08, &input),
        9 => aoc::solve_day!(day09, &input),
        10 => aoc::solve_day!(day10, &input),
        // 13 => aoc::solve_day!(day13, &input),
        11 => aoc::solve_day!(day11, &input),
        _ => println!("day not solved: {}", day),
    }
}
