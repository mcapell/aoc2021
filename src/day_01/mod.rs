use std::fs;

pub fn run() {
    let numbers = fs::read_to_string("data/day_01_1.txt")
        .expect("unable to read file")
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|w| w.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    println!("{}", first(&numbers));
    println!("{}", second(&numbers));
}

fn first(input: &[i32]) -> i32 {
    input.windows(2).filter(|g| g[0] < g[1]).count() as i32
}

fn second(input: &[i32]) -> i32 {
    input
        .windows(4)
        .filter(|g| g[0..3].iter().sum::<i32>() < g[1..4].iter().sum::<i32>())
        .count() as i32
}

#[test]
fn test_first() {
    let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(7, first(&input));
}

#[test]
fn test_second() {
    let input = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(5, second(&input));
}
