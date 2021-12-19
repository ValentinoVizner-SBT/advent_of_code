fn max_bit(nums: &[u32], bit: usize) -> u32 {
    let mut c = [0, 0];
    for &x in nums {
        c[(x as usize >> bit) & 1] += 1
    }
    (c[1] >= c[0]) as u32
}

pub fn part_one(input: &str) -> u32 {
    let numbers = input
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();
    let x = (0..12).map(|i| max_bit(&numbers, i) << i).sum::<u32>();
    x * (!x & 0xfff)
}

fn oxygen_func(input: &str, oxygen: u32) -> u32 {
    let lines = input
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();
    let mut nums = lines.to_vec();
    for i in (0..12).rev() {
        let keep = max_bit(&nums, i) ^ oxygen;
        nums.retain(|x| (x >> i) & 1 == keep);
        if nums.len() == 1 {
            break;
        }
    }
    nums[0]
}

pub fn part_two(input: &str) -> u32 {
    oxygen_func(input, 1) * oxygen_func(input, 0)
}

#[test]
fn test_part_one() {
    use aoc::utils::read_file;
    let input = read_file("examples", 3);
    assert_eq!(part_one(&input), 89606);
}

// #[test]
// fn test_part_two() {
//     use aoc::utils::read_file;
//     let input = read_file("examples", 3);
//     assert_eq!(part_two(&input), 230);
// }
