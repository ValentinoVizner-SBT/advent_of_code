use std::usize;
use itertools::{Itertools};
use solutions::utils::read_file;

fn char_handler(stack: &mut Vec<char>, chunk: char) -> Option<usize> {
    match chunk {
        '('|'{'|'['|'<' => stack.push(chunk), 
        _ => match (stack.pop(), chunk) {
            (Some('('), ')') => {},
            (Some('['), ']') => {},
            (Some('{'), '}') => {},
            (Some('<'), '>') => {},
            (_, ')') => return Some(3),
            (_, ']') => return Some(57),
            (_, '}') => return Some(1197),
            (_, '>') => return Some(25137),
            _ => unreachable!("Oops!"),
        },
    }
    None
}


fn syntax_scoring(s: &str) -> usize {
    let mut stack = Vec::new();
    s.chars().find_map(|c| char_handler(&mut stack, c)).unwrap_or(0)
}


fn autocomplete(string: &str) -> Option<usize> {
    let mut stack = Vec::new();
    if string.chars().map(|chunk| char_handler(&mut stack, chunk)).any(|opt| opt.is_some()) {
      return None;
    }
    let score = stack.iter().rev().fold(0, |score,chunk| match chunk {
      '(' => score * 5 + 1,
      '[' => score * 5 + 2,
      '{' => score * 5 + 3,
      '<' => score * 5 + 4,
      _ => unreachable!()
    });
    Some(score)
  }

solutions::main! {
    let p1 = read_file("/Users/valentinovizner/Documents/private_projects/rust/advent_of_code/rust_2021/input/2021/day10.txt").lines().map(syntax_scoring).sum::<usize>();
    let fixed_lines = read_file("/Users/valentinovizner/Documents/private_projects/rust/advent_of_code/rust_2021/input/2021/day10.txt").lines()
    .filter_map(autocomplete)
    .sorted()
    .collect::<Vec<_>>();
    let p2 = fixed_lines[fixed_lines.len() / 2]; // Middle score
    (p1, p2)

}
