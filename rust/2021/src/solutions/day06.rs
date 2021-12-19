use std::collections::HashMap;

pub struct Counter {
    memo: HashMap<(u8, usize), usize>,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            memo: HashMap::new(),
        }
    }

    fn count_bebes(&mut self, days_left: u8, total_time: usize) -> usize {
        if usize::from(days_left) + 1 > total_time {
            return 1;
        }

        // :: If we already encountered this pair before -> return the memoized value.
        if let Some(bebes) = self.memo.get(&(days_left, total_time)) {
            return *bebes;
        }

        // :: Else, we invoke the recursion :(
        let this_fish = self.count_bebes(6, total_time - usize::from(days_left) - 1);
        // :: The new lanternfish starts with an internal timer of 8 and does not start counting down until the next day.
        let new_fish = self.count_bebes(8, total_time - usize::from(days_left) - 1);

        let total_bebes = this_fish + new_fish;

        // :: Cache the result.
        self.memo.insert((days_left, total_time), total_bebes);

        total_bebes
    }
}
pub fn part_one(input: &str) -> usize {
    let nums = input
        .split(',')
        .map(|d| d.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    const NUMBER_OF_DAYS: usize = 80;
    let mut counter = Counter::new();

    nums.iter()
        .map(|d| counter.count_bebes(*d, NUMBER_OF_DAYS))
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let nums = input
        .split(',')
        .map(|d| d.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    const NUMBER_OF_DAYS: usize = 256;
    let mut counter = Counter::new();

    nums.iter()
        .map(|d| counter.count_bebes(*d, NUMBER_OF_DAYS))
        .sum()
}

#[test]
fn test_part_one() {
    use aoc::utils::read_file;
    let input = read_file("examples", 6);
    assert_eq!(part_one(&input), 5934);
}

#[test]
fn test_part_two() {
    use aoc::utils::read_file;
    let input = read_file("examples", 6);
    assert_eq!(part_two(&input), 26984457539);
}
