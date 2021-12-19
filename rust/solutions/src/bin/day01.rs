pub fn day01(input_file: Vec<String>) {
    let nums: Vec<i32> = input_file.iter().map(|num| num.parse().unwrap()).collect();

    println!("Day 01 - Part 1: {}", increasing(&nums, 1));
    println!("Day 01 - Part 2: {}", increasing(&nums, 3));

}

fn increasing(nums: &[i32], offset: usize) -> usize {
    nums.windows(offset + 1)
        .map(|x| (x[0] < x[offset]) as usize)
        .sum() // fold(0, |a, b| a + b)
}


#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    #[test]
    fn part1_example() {
        assert_eq!(
            increasing(&INPUT, 1),
            7
        );
    }
    #[test]
    fn part2_example() {
        assert_eq!(
            increasing(&INPUT, 3),
            5
        );
    }

}