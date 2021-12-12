use std::collections::{HashMap, HashSet};
use std::fs;

pub fn run() {
    let input = fs::read_to_string("data/day_08_1.txt")
        .expect("unable to read file")
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(parse_entry)
        .collect::<Vec<Entry>>();
    println!("{}", first(&input));
    println!("{}", second(&input));
}

fn first(input: &[Entry]) -> u32 {
    let lens: HashSet<usize> = HashSet::from([2, 3, 4, 7]);
    input
        .iter()
        .map(|e| {
            let left = e
                .unique
                .iter()
                .filter(|e| lens.contains(&e.len()))
                .map(hash)
                .collect::<HashSet<u8>>();

            e.output
                .iter()
                .map(hash)
                .filter(|o| left.contains(o))
                .count() as u32
        })
        .sum::<u32>()
}

fn second(input: &[Entry]) -> u32 {
    input
        .iter()
        .map(|e| {
            let map = identify(&e.unique);
            e.output
                .iter()
                .map(|v| format!("{}", map[&hash(v)]))
                .collect::<String>()
                .parse::<u32>()
                .expect("expected a number")
        })
        .sum()
}

fn hash(value: &String) -> u8 {
    value.chars().map(|c| 0x1 << (c as u8 - b'a')).sum()
}

fn identify(entries: &[String]) -> HashMap<u8, u8> {
    // Map easy ones
    let mut identities = entries
        .iter()
        .map(|v| match v.len() {
            2 => (hash(v), 1),
            3 => (hash(v), 7),
            4 => (hash(v), 4),
            7 => (hash(v), 8),
            _ => (0, 0),
        })
        .filter(|(k, _)| k != &0)
        .collect::<HashMap<u8, u8>>();

    // 9, 0 and 6
    let h_four = identities
        .iter()
        .filter(|(_, &v)| v == 4)
        .map(|(k, _)| k.to_owned())
        .collect::<Vec<u8>>()[0];
    let h_one = identities
        .iter()
        .filter(|(_, &v)| v == 1)
        .map(|(k, _)| k.to_owned())
        .collect::<Vec<u8>>()[0];
    entries.iter().for_each(|v| {
        if v.len() == 6 {
            let h = hash(v);
            if h & h_four == h_four {
                identities.insert(h, 9);
            } else if h & h_one == h_one {
                identities.insert(h, 0);
            } else {
                identities.insert(h, 6);
            }
        }
    });

    // 2, 3 and 5
    let h_nine = identities
        .iter()
        .filter(|(_, &v)| v == 9)
        .map(|(k, _)| k.to_owned())
        .collect::<Vec<u8>>()[0];
    entries.iter().for_each(|v| {
        if v.len() == 5 {
            let h = hash(v);
            if h & h_one == h_one {
                identities.insert(h, 3);
            } else if h & h_nine == h {
                identities.insert(h, 5);
            } else {
                identities.insert(h, 2);
            }
        }
    });

    identities
}

#[derive(Debug)]
struct Entry {
    unique: Vec<String>,
    output: Vec<String>,
}

fn parse_entry(line: &str) -> Entry {
    let sp = line.split('|').collect::<Vec<&str>>();
    Entry {
        unique: sp[0]
            .trim()
            .split(' ')
            .map(|s| s.to_owned())
            .collect::<Vec<String>>(),
        output: sp[1]
            .trim()
            .split(' ')
            .map(|s| s.to_owned())
            .collect::<Vec<String>>(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_first() {
        assert_eq!(
            26,
            first(&INPUT.split('\n').map(parse_entry).collect::<Vec<Entry>>())
        );
    }

    #[test]
    fn test_second() {
        assert_eq!(
            61229,
            second(&INPUT.split('\n').map(parse_entry).collect::<Vec<Entry>>())
        );
    }
}
