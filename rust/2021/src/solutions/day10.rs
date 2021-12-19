pub fn score(line: &str) -> (i32, i64) {
    let mut expect: Vec<char> = Vec::new();

    for c in line.chars() {
        match c {
            '(' => expect.push(')'),
            '{' => expect.push('}'),
            '[' => expect.push(']'),
            '<' => expect.push('>'),
            _ => {
                if c != expect.pop().unwrap() {
                    match c {
                        ')' => return (3, 0),
                        ']' => return (57, 0),
                        '}' => return (1197, 0),
                        '>' => return (25137, 0),
                        _ => panic!("illegal char {}", c),
                    }
                }
            }
        }
    }

    let mut total: i64 = 0;
    while !expect.is_empty() {
        total *= 5;
        total += match expect.pop().unwrap() {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            c => panic!("illegal char {}", c),
        };
    }

    (0, total)
}

pub fn part_one(input: &str) -> i32 {
    let lines: Vec<String> = input.lines().map(str::to_string).collect();
    let result: Vec<(i32, i64)> = lines.iter().map(|line| score(line)).collect();
    result.iter().fold(0, |sum, tuple| sum + tuple.0)
}

pub fn part_two(input: &str) -> i64 {
    let lines: Vec<String> = input.lines().map(str::to_string).collect();
    let result: Vec<(i32, i64)> = lines.iter().map(|line| score(line)).collect();
    let mut part2: Vec<i64> = result
    .iter()
    .filter(|x| x.1 != 0)
    .map(|tuple| tuple.1)
    .collect();
    part2.sort_unstable();
    part2[part2.len() / 2]
}

#[test]
fn test_part_one() {
    use aoc::utils::read_file;
    let input = read_file("examples", 10);
    assert_eq!(part_one(&input), 26397);
}

#[test]
fn test_part_two() {
    use aoc::utils::read_file;
    let input = read_file("examples", 10);
    assert_eq!(part_two(&input), 288957);
}
