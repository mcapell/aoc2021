use std::fs;

pub fn run() {
    let report = fs::read_to_string("data/day_03_1.txt")
        .expect("unable to read file")
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
    println!("{}", first(&report));
    println!("{}", second(&report));
}

fn first(input: &[String]) -> i32 {
    let d = parse_diagnostics(&transpose(input));
    i32::from_str_radix(d.gamma.as_str(), 2).expect("expected a binary value")
        * i32::from_str_radix(d.epsilon.as_str(), 2).expect("expected a binary value")
}

fn second(input: &[String]) -> i32 {
    let l = life_support(input);
    l.oxigen * l.co2
}

struct Diagnostic {
    gamma: String,
    epsilon: String,
}

fn parse_diagnostics(report: &[String]) -> Diagnostic {
    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();
    for diagnostic in report {
        let one = diagnostic.chars().filter(|&c| c == '1').count();
        if one > diagnostic.chars().count() - one {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    Diagnostic {
        gamma,
        epsilon,
    }
}

fn transpose(input: &[String]) -> Vec<String> {
    let mut report: Vec<String> = vec!["".to_string(); input[0].len()];
    for diagnostic in input {
        for (i, ch) in diagnostic.chars().enumerate() {
            report[i].push(ch);
        }
    }
    report
}

struct LifeSupport {
    oxigen: i32,
    co2: i32,
}

#[allow(clippy::all)]
fn life_support(input: &[String]) -> LifeSupport {
    let size = input[0].len() as i32;
    let mut start = "".to_string();
    let mut result = input.clone().to_owned();
    for i in 0..size {
        let one = result
            .iter()
            .filter(|c| c.chars().nth(i as usize).unwrap() == '1')
            .count() as i32;

        start = format!(
            "{}{}",
            start,
            if one >= (result.len() as i32 - one) {
                "1"
            } else {
                "0"
            }
        );

        result.retain(|l| l.starts_with(&start));
        if result.len() == 1 {
            break;
        }
    }
    let oxigen = result[0].to_owned();

    let mut result = input.clone().to_owned();
    start = "".to_string();
    for i in 0..size {
        let one = result
            .iter()
            .filter(|c| c.chars().nth(i as usize).unwrap() == '1')
            .count() as i32;
        start = format!(
            "{}{}",
            start,
            if one < (result.len() as i32 - one) {
                "1"
            } else {
                "0"
            }
        );

        result.retain(|l| l.starts_with(&start));
        if result.len() == 1 {
            break;
        }
    }
    let co2 = result[0].to_owned();

    LifeSupport {
        oxigen: i32::from_str_radix(oxigen.as_str(), 2).expect("expected a binary value"),
        co2: i32::from_str_radix(co2.as_str(), 2).expect("expected a binary value"),
    }
}

#[test]
fn test_first() {
    let input =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"
            .split('\n')
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
    assert_eq!(198, first(&input));
}

#[test]
fn test_second() {
    let input =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"
            .split('\n')
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
    assert_eq!(230, second(&input));
}
