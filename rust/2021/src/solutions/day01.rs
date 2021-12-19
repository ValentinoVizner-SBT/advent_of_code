// identify value increases across iterator by using a sliding window.
fn increasing(nums: &[i32], offset: usize) -> usize {
    nums.windows(offset + 1)
        .map(|x| (x[0] < x[offset]) as usize)
        .sum() // fold(0, |a, b| a + b)
}

pub fn part_one(input: &str) -> usize {
    let nums: Vec<i32> = input.lines().map(|num| num.parse().unwrap()).collect();
    increasing(&nums, 1)
}

pub fn part_two(input: &str) -> usize {
    let nums: Vec<i32> = input.lines().map(|num| num.parse().unwrap()).collect();
    increasing(&nums, 3)
}

#[test]
fn test_part_one() {
    use aoc::utils::read_file;
    let input = read_file("examples", 1);
    assert_eq!(part_one(&input), 7);
}

#[test]
fn test_part_two() {
    use aoc::utils::read_file;
    let input = read_file("examples", 1);
    assert_eq!(part_two(&input), 5);
}
