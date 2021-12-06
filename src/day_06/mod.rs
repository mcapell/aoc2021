use std::fs;
use std::collections::HashMap;

pub fn run() {
    let values = parse(fs::read_to_string("data/day_06_1.txt")
        .expect("unable to read file")
        .trim());

    println!("{}", predict(values.clone(), 80));
    println!("{}", predict(values, 256));
}

fn parse(input: &str) -> HashMap<usize, u64> {
    input.split(',').fold(HashMap::new(), |mut k, n| {
        let e = k.entry(n.parse::<usize>().expect("expected a number")).or_insert(0);
        *e += 1;
        k
    })
}

fn predict(mut input: HashMap<usize, u64>, days: usize) -> u64 {
    for _ in 0..days {
        input = input.iter().fold(HashMap::new(), |mut res, (&k, v)| {
            if k == 0 {
                *res.entry(6).or_insert(0) += v;
                *res.entry(8).or_insert(0) += v;
            } else {
                *res.entry(k - 1).or_insert(0) += v;
            }
            res
        });
    }

    input.iter().map(|(_, &v)| v).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_first() {
        let input = parse(INPUT);
        assert_eq!(26, predict(input.clone(), 18));
        assert_eq!(5934, predict(input.clone(), 80));
    }

    #[test]
    fn test_second() {
        let input = parse(INPUT);
        assert_eq!(26984457539, predict(input.clone(), 256));
    }
}
