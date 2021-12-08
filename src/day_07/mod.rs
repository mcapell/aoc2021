use std::fs;

pub fn run() {
    let mut input = fs::read_to_string("data/day_07_1.txt")
        .expect("unable to read file")
        .trim()
        .split(',')
        .map(|n| n.parse::<i32>().expect("expected a number"))
        .collect::<Vec<i32>>();
    input.sort_unstable();

    println!("{}", fuel_usage(&input, noop));
    println!("{}", fuel_usage(&input, sum_iter));
}

fn fuel_usage(input: &[i32], f: fn(i32) -> i32) -> i32 {
    let mut cost = i32::MAX;
    for i in 0..input.len() {
        let new_cost = input.iter().map(|n| f((n - i as i32).abs())).sum();
        if new_cost > cost {
            break
        }
        cost = new_cost;
    }

    cost
}

fn noop(n: i32) -> i32 {
    n
}

fn sum_iter(n: i32) -> i32 {
    if n == 0 {
        return n;
    }
    n + sum_iter(n - 1)
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &[i32] = &[0, 1, 1, 2, 2, 2, 4, 7, 14, 16];

    #[test]
    fn test_first() {
        assert_eq!(37, fuel_usage(&INPUT, noop));
    }

    #[test]
    fn test_second() {
        assert_eq!(168, fuel_usage(&INPUT, sum_iter));
    }
}
