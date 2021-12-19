use aoc::utils::read_file;
use itertools::Itertools;
use std::collections::HashSet;

// static FOLDS: &[String] = read_file("examples", 13)
//     .lines()
//     .map(|f| f.starts_with("fold"));

fn fold_grid(points: &HashSet<(i32, i32)>, (dir, pos): (char, i32)) -> HashSet<(i32, i32)> {
    points
        .iter()
        .map(|&(x, y)| match (dir, x, y) {
            ('x', x, y) if x < pos => (x, y),
            ('x', x, y) => (pos * 2 - x, y),
            ('y', x, y) if y < pos => (x, y),
            ('y', x, y) => (x, pos * 2 - y),
            _ => unreachable!(),
        })
        .collect()
}

pub fn part_one(input: &str) -> u32 {
    let mut points: Vec<(i64, i64)> = Vec::new();
    let mut folds: Vec<String> = Vec::new();
    let lines: Vec<String> = read_file("examples", 13)
        .lines()
        .map(|f| f.parse().unwrap())
        .collect();
    for line in lines {
        if line.contains("fold along ") {
            let res: Vec<String> = line.split('=').map(|s| s.to_string()).collect();
            println!("{:?}", res);
        }
    }

    0
}

pub fn part_two(input: &str) -> u32 {
    0
}

#[test]
fn test_part_one() {
    use aoc::utils::read_file;
    let input = read_file("examples", 13);
    assert_eq!(part_one(&input), 17);
}

//
// #[test]
// fn test_part_two() {
//     use aoc::utils::read_file;
//     let input = parse(read_file("examples", 13));
//     assert_eq!(part_two(&input), 0);
// }
