use std::collections::{HashMap, HashSet};
use std::fs;

pub fn run() {
    let input = fs::read_to_string("data/day_04_1.txt")
        .expect("unable to read file")
        .split('\n')
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    println!("{}", first(&input));
    println!("{}", second(&input));
}

fn first(input: &[String]) -> u32 {
    let mut game = parse_input(input);
    let res = game.run_until_winner();
    res.0 * res.1
}

fn second(input: &[String]) -> u32 {
    let mut game = parse_input(input);
    let res = game.run_until_last();
    res.0 * res.1
}

#[derive(Debug)]
struct Board {
    id: usize,
    numbers: HashMap<u32, usize>,
    drawn: [[usize; 5]; 5],
}

impl Board {
    fn is_winner(&self) -> bool {
        self.get_winner_row().is_some()
    }

    fn get_winner_row(&self) -> Option<u32> {
        for l in self.drawn.iter() {
            if l.iter().sum::<usize>() >= 5 {
                return Some(self.sum_unmarked());
            }
        }
        for n in 0..5 {
            if self.drawn.iter().map(|l| l[n]).sum::<usize>() >= 5 {
                return Some(self.sum_unmarked());
            }
        }
        None
    }

    fn sum_unmarked(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|(_k, &p)| self.drawn[p / 5][p % 5] == 0)
            .map(|(&k, _)| k)
            .sum()
    }

    fn mark_number(&mut self, number: u32) {
        if let Some(pos) = self.numbers.get(&number) {
            self.drawn[pos / 5][pos % 5] = 1;
        }
    }
}

#[derive(Debug)]
struct Game {
    numbers: Vec<u32>,
    boards: Vec<Board>,
}

impl Game {
    fn run_until_winner(&mut self) -> (u32, u32) {
        for n in self.numbers.iter() {
            for board in &mut self.boards {
                board.mark_number(*n);
                if board.is_winner() {
                    return (board.get_winner_row().unwrap(), *n);
                }
            }
        }
        (0, 0)
    }

    fn run_until_last(&mut self) -> (u32, u32) {
        let size = self.boards.len();
        let mut finished: HashSet<usize> = HashSet::new();
        for n in self.numbers.iter() {
            for board in &mut self.boards {
                if finished.contains(&board.id) {
                    continue;
                }
                board.mark_number(*n);
                if board.is_winner() {
                    finished.insert(board.id);
                }
                if finished.len() == size {
                    return (board.get_winner_row().unwrap(), *n);
                }
            }
        }
        (0, 0)
    }
}

fn parse_input(input: &[String]) -> Game {
    let mut game = Game {
        numbers: input[0]
            .split(',')
            .map(|n| n.parse::<u32>().expect("numeric expected"))
            .collect(),
        boards: vec![],
    };
    let mut board = Board {
        id: 0,
        numbers: HashMap::new(),
        drawn: [[0; 5]; 5],
    };
    let mut col = 0;
    for line in input.iter().skip(2) {
        if line.is_empty() {
            game.boards.push(board);
            board = Board {
                id: game.boards.len(),
                numbers: HashMap::new(),
                drawn: [[0; 5]; 5],
            };
            col = 0;
            continue;
        }
        for (i, n) in line
            .split(' ')
            .map(|n| n.trim())
            .filter(|n| !n.is_empty())
            .map(|n| n.trim().parse::<u32>().expect("numberic expected"))
            .enumerate()
        {
            board.numbers.insert(n, i + col);
        }
        col += 5;
    }

    if !board.numbers.is_empty() {
        game.boards.push(board);
    }

    game
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
 ";

    #[test]
    fn test_first() {
        let clean_input = INPUT
            .split('\n')
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        assert_eq!(4512, first(&clean_input));
    }

    #[test]
    fn test_second() {
        let clean_input = INPUT
            .split('\n')
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        assert_eq!(1924, second(&clean_input));
    }
}
