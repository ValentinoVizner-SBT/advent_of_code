use itertools::Itertools;

fn input_to_bytes(input: String) -> Vec<(u8, i32)> {
    input
        .split_whitespace()
        .tuples()
        .map(|(op, i)| (op.as_bytes()[0], i.parse().unwrap()))
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> i32 {
    let ops = input_to_bytes(input.to_string());
    let (x, y) = ops.iter().fold((0, 0), |(x, y), (op, i)| match op {
        b'f' => (x + i, y),
        b'd' => (x, y + i),
        b'u' => (x, y - i),
        _ => unreachable!("Got strange command!"),
    });
    x * y
}

pub fn part_two(input: &str) -> i32 {
    let ops = input_to_bytes(input.to_string());

    let (x, y, _) = ops.iter().fold((0, 0, 0), |(x, y, aim), (op, i)| match op {
        b'f' => (x + i, y + aim * i, aim),
        b'd' => (x, y, aim + i),
        b'u' => (x, y, aim - i),
        _ => unreachable!(),
    });
    x * y
}

#[test]
fn test_part_one() {
    use aoc::utils::read_file;
    let input = read_file("examples", 2);
    assert_eq!(part_one(&input), 150);
}

#[test]
fn test_part_two() {
    use aoc::utils::read_file;
    let input = read_file("examples", 2);
    assert_eq!(part_two(&input), 900);
}
