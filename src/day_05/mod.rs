use std::cmp;
use std::fs;

pub fn run() {
    let lines = fs::read_to_string("data/day_05_1.txt")
        .expect("unable to read file")
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
    let input = parse_input(&lines);

    println!("{}", first(&input));
    println!("{}", second(&input));
}

fn first(input: &[Line]) -> usize {
    sum_intersections(parse_vents(input, false))
}

fn second(input: &[Line]) -> usize {
    sum_intersections(parse_vents(input, true))
}

fn sum_intersections(grid: Grid) -> usize {
    grid.iter().map(|r| r.iter().filter(|&c| c > &1).count()).sum()
}

type Grid = Vec<Vec<usize>>;

struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

fn parse_input(input: &[String]) -> Vec<Line> {
    input
        .iter()
        .map(|l| {
            let sp = l
                .split(" -> ")
                .map(|p| {
                    p.split(',')
                        .map(|s| s.parse::<i32>().expect("expected number"))
                        .take(2)
                        .collect()
                })
                .collect::<Vec<Vec<i32>>>();
            let (x1, y1, x2, y2) = (sp[0][0], sp[0][1], sp[1][0], sp[1][1]);

            let rev = x1 > x2 || x1 == x2 && y1 > y2;
            Line {
                x1: if rev { x2 } else { x1 },
                y1: if rev { y2 } else { y1 },
                x2: if rev { x1 } else { x2 },
                y2: if rev { y1 } else { y2 },
            }
        })
        .collect()
}

fn parse_vents(lines: &[Line], diagonal: bool) -> Grid {
    let max_x = lines
        .iter()
        .map(|l| cmp::max(l.x1, l.x2))
        .max()
        .unwrap() as usize;
    let max_y = lines
        .iter()
        .map(|l| cmp::max(l.y1, l.y2))
        .max()
        .unwrap() as usize;
    let mut grid = vec![vec![0; max_x + 1]; max_y + 1];

    for line in lines {
        if line.x1 == line.x2 {
            for y in line.y1..=line.y2 {
                grid[y as usize][line.x1 as usize] += 1;
            }
        } else if line.y1 == line.y2 {
            for x in line.x1..=line.x2 {
                grid[line.y1 as usize][x as usize] += 1;
            }
        } else if diagonal {
            let mut x = line.x1;
            let mut y = line.y1;
            for _ in 0..=(line.x2 - line.x1) {
                grid[y as usize][x as usize] += 1;
                x += 1;
                if line.y1 < line.y2 {
                    y += 1;
                } else {
                    y -= 1;
                }
            }
        }
    }

    grid
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

    #[test]
    fn test_first() {
        let input = parse_input(
            &INPUT
                .split('\n')
                .filter(|l| !l.is_empty())
                .map(|l| l.to_string())
                .collect::<Vec<String>>(),
        );
        assert_eq!(5, first(&input));
    }

    #[test]
    fn test_second() {
        let input = parse_input(
            &INPUT
                .split('\n')
                .filter(|l| !l.is_empty())
                .map(|l| l.to_string())
                .collect::<Vec<String>>(),
        );
        assert_eq!(12, second(&input));
    }
}
