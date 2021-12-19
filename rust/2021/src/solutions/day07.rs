pub fn part_one(input: &str) -> i64 {
    let crabs: Vec<i64> = input
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    let mut best = i64::MAX;

    for meeting_point in min..=max {
        let mut score = 0;
        for crab in &crabs {
            score += (crab - meeting_point).abs();
        }

        best = best.min(score);
    }

    best
}

pub fn part_two(input: &str) -> i64 {
    let crabs: Vec<i64> = input
        .trim()
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect();

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();
    let mut best = i64::MAX;

    for meeting_point in min..=max {
        let mut score = 0;
        for crab in &crabs {
            let delta = (crab - meeting_point).abs();
            score += delta * (delta + 1) / 2;
        }

        best = best.min(score);
    }

    best
}

#[test]
fn test_part_one() {
    use aoc::utils::read_file;
    let input = read_file("examples", 7);
    assert_eq!(part_one(&input), 37);
}

#[test]
fn test_part_two() {
    use aoc::utils::read_file;
    let input = read_file("examples", 7);
    assert_eq!(part_two(&input), 168);
}
