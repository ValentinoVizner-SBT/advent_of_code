use std::collections::HashSet;
use itertools::Itertools;
use solutions::utils::read_file;


fn part1(input: &[Vec<u8>]) -> usize {
  let mut ans = 0;
  for (x,y) in (0..input[0].len()).cartesian_product(0..input.len()) {
    let is_low = [(x-1,y),(x+1,y),(x,y-1),(x,y+1)].iter()
      .filter_map(|&(x,y)| input.get(y as usize).and_then(|line| line.get(x as usize)))
      .all(|&i| i > input[y][x]);
    if is_low { ans += input[y][x] as usize + 1; }
  }
  ans
}

fn remove_component((x,y): (usize,usize), coords: &mut HashSet<(usize,usize)>) -> usize {
  if !coords.remove(&(x,y)) {
    return 0;
  }
  1 + [(x-1,y),(x+1,y),(x,y-1),(x,y+1)].iter()
    .map(|&neighbour| remove_component(neighbour, coords))
    .sum::<usize>()
}

fn part2(input: &[Vec<u8>]) -> usize {
  let mut points = (0..input[0].len()).cartesian_product(0..input.len())
    .filter(|&(x,y)| input[y][x] != 9)
    .collect::<HashSet<_>>();
  let mut cs = vec![];
  while let Some(&p) = points.iter().next() {
    cs.push(remove_component(p, &mut points));
  }
  cs.iter().sorted().rev().take(3).product()
}

solutions::main! {
    let input = read_file("/Users/valentinovizner/Documents/private_projects/rust/advent_of_code/rust_2021/input/2021/day9.txt").lines()
      .map(|l| l.bytes().map(|c| c - b'0').collect())
      .collect::<Vec<_>>();
    (part1(&input), part2(&input))
}