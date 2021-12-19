pub mod utils;

#[macro_export]
macro_rules! solve_day {
    ($day:path, $input:expr) => {{
        use $day::*;
        println!("----");
        static ANSI_BOLD: &str = "\x1b[1m";
        static ANSI_RESET: &str = "\x1b[0m";
        println!("");
        println!("🎄 {}Part 1{} 🎄", ANSI_BOLD, ANSI_RESET);
        println!("");
        utils::print_result(part_one, $input);
        println!("");
        println!("🎄 {}Part 2{} 🎄", ANSI_BOLD, ANSI_RESET);
        println!("");
        utils::print_result(part_two, $input);
        println!("");
        println!("----");
    }};
}
