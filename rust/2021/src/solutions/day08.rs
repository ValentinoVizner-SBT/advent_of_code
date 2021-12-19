use std::collections::HashSet;
use std::iter::FromIterator;

// digit    seg             seg     count
// 0        6               a       8   *
// 1        2               b       6
// 2        5               c       8   *
// 3        5               d       7   *
// 4        4               e       4
// 5        5               f       9
// 6        6               g       7   *
// 7        3
// 8        7
// 9        6

pub fn part_one(input: &str) -> usize {
    input.lines().fold(0, |sum, line| sum + part_1(line))
}

pub fn part_two(input: &str) -> usize {
    input.lines().fold(0, |sum, line| sum + decode(line))
}

fn part_1(line: &str) -> usize {
    let code = line.split('|').nth(1).unwrap();
    let digits: Vec<String> = code.split_whitespace().map(|x| x.to_string()).collect();
    let mut total = 0;
    for d in digits {
        total += match d.len() {
            2 | 4 | 3 | 7 => 1,
            _ => 0,
        };
    }
    total
}

fn decode(line: &str) -> usize {
    let mut split = line.split('|');
    let values = split.next().unwrap();
    let digits: Vec<String> = values.split_whitespace().map(|x| x.to_string()).collect();
    let mut counts = vec![0i32; 7];
    let mut hsetlist: Vec<HashSet<char>> = Vec::new();
    let mut is1 = 0;
    let mut is4 = 0;
    let mut is7 = 0;
    for (i, d) in digits.iter().enumerate() {
        let mut hs: HashSet<char> = HashSet::new();
        for c in d.chars() {
            hs.insert(c);
            counts[c as usize - 'a' as usize] += 1;
        }
        if hs.len() == 2 {
            is1 = i
        } else if hs.len() == 3 {
            is7 = i
        } else if hs.len() == 4 {
            is4 = i;
        }
        hsetlist.push(hs);
    }

    let mut check: HashSet<char> = HashSet::new();
    check.insert('a');
    check.insert('b');
    // println!("are equal? {:?}", check == hsetlist[is1]);

    // Get a by difference of 7 and 1
    let seg_a_hash: HashSet<_> = hsetlist[is7].difference(&hsetlist[is1]).collect();
    let seg_a = **seg_a_hash.iter().next().unwrap();

    // Get d by difference of 4 and 1
    let mut seg_d_hash: HashSet<_> = hsetlist[is4].difference(&hsetlist[is1]).collect();
    // println!("seg_d = {:?}", seg_d_hash);

    let mut seg_b: char = '?';
    let mut seg_c: char = '?';
    let mut seg_d: char = '?';
    let mut seg_e: char = '?';
    let mut seg_f: char = '?';

    // println!("counts = {:?}", counts);

    let mut sevens: HashSet<char> = HashSet::new();
    for (i, c) in counts.iter().enumerate() {
        if *c == 6 {
            seg_b = ('a' as usize + i) as u8 as char;
            seg_d_hash.remove(&seg_b);
            seg_d = **seg_d_hash.iter().next().unwrap();
        } else if *c == 4 {
            seg_e = ('a' as usize + i) as u8 as char;
        } else if *c == 9 {
            seg_f = ('a' as usize + i) as u8 as char;
        } else if *c == 8 {
            let tmp = ('a' as usize + i) as u8 as char;
            if tmp != seg_a {
                seg_c = tmp;
            }
        } else if *c == 7 {
            sevens.insert(('a' as usize + i) as u8 as char);
        }
    }
    sevens.remove(&seg_d);
    let seg_g = *sevens.iter().next().unwrap();

    // println!(
    //     "seg_a = {}, seg_b = {}, seb_c = {}, seg_d = {}, seg_e = {}, seg_g ={}, seg_g ={}",
    //     seg_a, seg_b, seg_c, seg_d, seg_e, seg_f, seg_g
    // );
    // let zero_test: HashSet<char> = HashSet::from_iter(
    //     [seg_a, seg_b, seg_c, seg_e, seg_f, seg_g]
    //         .iter()
    //         .map(|x| *x),
    // );
    // println!("yero ? {:?}", zero_test);

    let zero: HashSet<char> = HashSet::from([seg_a, seg_b, seg_c, seg_e, seg_f, seg_g]);
    let one: HashSet<char> = HashSet::from([seg_c, seg_f]);
    let two: HashSet<char> = HashSet::from([seg_a, seg_c, seg_d, seg_e, seg_g]);
    let three: HashSet<char> = HashSet::from([seg_a, seg_c, seg_d, seg_f, seg_g]);
    let four: HashSet<char> = HashSet::from([seg_b, seg_c, seg_d, seg_f]);
    let five: HashSet<char> = HashSet::from([seg_a, seg_b, seg_d, seg_f, seg_g]);
    let six: HashSet<char> = HashSet::from([seg_a, seg_b, seg_d, seg_e, seg_f, seg_g]);
    let seven: HashSet<char> = HashSet::from([seg_a, seg_c, seg_f]);
    let eight: HashSet<char> = HashSet::from([seg_a, seg_b, seg_c, seg_d, seg_e, seg_f, seg_g]);
    let nine: HashSet<char> = HashSet::from([seg_a, seg_b, seg_c, seg_d, seg_f, seg_g]);

    let decoder_ring = vec![zero, one, two, three, four, five, six, seven, eight, nine];

    let segments = split.next().unwrap();
    let mut result = 0;
    for d in segments.split_whitespace() {
        result *= 10;
        let hs: HashSet<char> = HashSet::from_iter(d.chars());
        for (i, code) in decoder_ring.iter().enumerate() {
            if hs == *code {
                result += i;
            }
        }
    }

    result
}

#[test]
fn test_part_one() {
    use aoc::utils::read_file;
    let input = read_file("examples", 8);
    assert_eq!(part_one(&input), 26);
}

#[test]
fn test_part_two() {
    use aoc::utils::read_file;
    let input = read_file("examples", 8);
    assert_eq!(part_two(&input), 61229);
}
