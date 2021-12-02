use std::fs;

pub fn run() {
    let commands = fs::read_to_string("data/day_02_1.txt")
        .expect("unable to read file")
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| create_command(l))
        .collect::<Vec<Command>>();

    println!("{}", first(&commands));
    println!("{}", second(&commands));
}

fn first(input: &[Command]) -> i32 {
    input
        .iter()
        .map(|c| match c.direction {
            Direction::Down => c.value,
            Direction::Up => -c.value,
            Direction::Forward => 0,
        })
        .sum::<i32>()
        * input
            .iter()
            .map(|c| match c.direction {
                Direction::Forward => c.value,
                _ => 0,
            })
            .sum::<i32>()
}

fn second(input: &[Command]) -> i32 {
    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;
    for cmd in input {
        match cmd.direction {
            Direction::Down => aim += cmd.value,
            Direction::Up => aim -= cmd.value,
            Direction::Forward => {
                horizontal += cmd.value;
                depth += aim * cmd.value;
            }
        }
    }
    depth * horizontal
}

enum Direction {
    Forward,
    Down,
    Up,
}

struct Command {
    direction: Direction,
    value: i32,
}

fn create_command(cmb: &str) -> Command {
    Command {
        direction: match cmb.split(' ').next().expect("expected the command value") {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => panic!("invalid command"),
        },
        value: cmb
            .split(' ')
            .nth(1)
            .expect("expected two values for command")
            .parse::<i32>()
            .expect("expected a numeric value"),
    }
}

#[test]
fn test_first() {
    let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2"
        .split('\n')
        .map(|l| create_command(l))
        .collect::<Vec<Command>>();
    assert_eq!(150, first(&input));
}

#[test]
fn test_second() {
    let input = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2"
        .split('\n')
        .map(|l| create_command(l))
        .collect::<Vec<Command>>();
    assert_eq!(900, second(&input));
}
