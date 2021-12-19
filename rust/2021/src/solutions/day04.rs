/// Board size (NxN)
struct Board {
    grid: Vec<i32>,
    size: usize,
    won: bool,
}

impl Board {
    pub fn new(grid: Vec<i32>, size: usize) -> Board {
        Board {
            grid,
            size,
            won: false,
        }
    }

    #[cfg(test)]
    pub fn get_grid(&self) -> &Vec<i32> {
        &self.grid
    }

    pub fn call_number(&mut self, called: i32) -> Option<i32> {
        if self.won {
            return None;
        }
        if let Some(loc) = self.grid.iter().position(|loc| *loc == called) {
            self.grid[loc] = -1;

            let row = loc / self.size;
            let col = loc % self.size;

            let mut rsum = 0;
            let mut csum = 0;
            for i in 0..self.size {
                rsum += self.grid[row * self.size + i];
                csum += self.grid[i * self.size + col];
            }

            if rsum == -5 || csum == -5 {
                // TODO: fix size for -5
                let sum: i32 = self.grid.iter().filter(|&x| *x != -1).sum();
                self.won = true;
                return Some(sum * called);
            }
        }

        None
    }
}

fn find_winners(numbers: Vec<i32>, boards: &mut Vec<Board>) -> (i32, i32) {
    let mut first_winner = None;
    let mut last_winner = 0;

    for call in numbers {
        for b in &mut *boards {
            if let Some(winner) = b.call_number(call) {
                if first_winner == None {
                    first_winner = Some(winner);
                }
                last_winner = winner;
            }
        }
    }

    (first_winner.unwrap(), last_winner)
}

fn parse_boards(data: &[String]) -> (Vec<i32>, Vec<Board>) {
    let numbers: Vec<i32> = data[0].split(',').map(|x| x.parse().unwrap()).collect();
    let mut boards: Vec<Board> = Vec::new();

    let mut index = 2;
    while index < data.len() {
        let mut grid = Vec::new();
        for i in 0..5 {
            let row: Vec<i32> = data[index + i]
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            for n in row {
                grid.push(n);
            }
        }
        boards.push(Board::new(grid, 5));
        index += 6;
    }

    (numbers, boards)
}

pub fn part_one(input: &str) -> i32 {
    let lines = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<String>>();
    let (nlist, mut blist) = parse_boards(&lines);
    let (first, last) = find_winners(nlist, &mut blist);
    first
}

pub fn part_two(input: &str) -> i32 {
    let lines = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<String>>();
    let (nlist, mut blist) = parse_boards(&lines);
    let (first, last) = find_winners(nlist, &mut blist);
    last
}

#[test]
fn test_part_one() {
    use aoc::utils::read_file;
    let input = read_file("examples", 4);
    assert_eq!(part_one(&input), 4512);
}

#[test]
fn test_part_two() {
    use aoc::utils::read_file;
    let input = read_file("examples", 4);
    assert_eq!(part_two(&input), 1924);
}
