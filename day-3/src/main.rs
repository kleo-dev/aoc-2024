use std::fs;
use regex::Regex;

pub fn parse(input: &str) -> Vec<(i32, i32)> {
    let pattern = Regex::new(r"^mul\((\d+),(\d+)\)").unwrap();
    let do_pattern = Regex::new(r"^do\(\)").unwrap();
    let dont_pattern = Regex::new(r"^don't\(\)").unwrap();

    let mut index: usize = 0;
    let mut do_mul = true;

    let mut parsed = Vec::new();

    while index < input.len() {
        if let Some(_) = do_pattern.captures(&input[index..]) {
            do_mul = true;
            index += 1;
        } else if let Some(_) = dont_pattern.captures(&input[index..]) {
            do_mul = false;
            index += 1;
        } else if let Some(caps) = pattern.captures(&input[index..]) {
            if do_mul {
                parsed.push((
                    caps.get(1).unwrap().as_str().parse().unwrap(),
                    caps.get(2).unwrap().as_str().parse().unwrap(),
                ));
                index += caps.len();
            } else {
                index += 1;
            }
        } else {
            index += 1;
        }
    }

    parsed
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    //let input = String::from("don't()mul(2,2)");
    
    let parsed = parse(&input);

    let mut sum = 0;

    for (x, y) in &parsed {
        sum += *x * *y;
    }

    //println!("parsed: {parsed:?}");
    println!("sum: {sum}");
}
