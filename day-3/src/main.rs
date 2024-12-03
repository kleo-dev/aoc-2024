use std::fs;

use regex::Regex;

pub fn parse(input: &str) -> Vec<(i32, i32)> {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    pattern.captures_iter(input).map(|caps| {
        (caps.get(1).unwrap().as_str().parse().unwrap(), caps.get(2).unwrap().as_str().parse().unwrap())
    }).collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let parsed = parse(&input);

    let mut sum = 0;

    for (x, y) in parsed {
        sum += x * y;
    }

    println!("{sum}");
}
