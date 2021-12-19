use itertools::Itertools;
use std::collections::HashMap;

fn num_overlapping(lines: impl Iterator<Item = (i32, i32, i32, i32)>) -> usize {
    let mut points = HashMap::new();
    for (x1, y1, x2, y2) in lines {
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();
        let (mut x, mut y) = (x1, y1);
        while (x, y) != (x2 + dx, y2 + dy) {
            *points.entry((x, y)).or_insert(0) += 1;
            x += dx;
            y += dy;
        }
    }
    points.values().filter(|&&n| n > 1).count()
}

pub fn part_one(input: &str) -> usize {
    let lines = input
        .lines()
        .filter_map(|l| {
            l.split(" -> ")
                .map(|s| s.split(','))
                .flatten()
                .map(|i| i.parse().unwrap())
                .collect_tuple()
        })
        .collect::<Vec<_>>();
    num_overlapping(
        lines
            .iter()
            .copied()
            .filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2),
    )
}

pub fn part_two(input: &str) -> usize {
    let lines = input
        .lines()
        .filter_map(|l| {
            l.split(" -> ")
                .map(|s| s.split(','))
                .flatten()
                .map(|i| i.parse().unwrap())
                .collect_tuple()
        })
        .collect::<Vec<_>>();
    num_overlapping(lines.iter().copied())
}

#[test]
fn test_part_one() {
    use aoc::utils::read_file;
    let input = read_file("examples", 5);
    assert_eq!(part_one(&input), 5);
}

#[test]
fn test_part_two() {
    use aoc::utils::read_file;
    let input = read_file("examples", 5);
    assert_eq!(part_two(&input), 12);
}
