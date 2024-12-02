use std::fs;

fn is_safe(report: Vec<i32>) -> bool {
    let mut order = 0;

    for index in 1..report.len() {
        let diff = report[index].checked_sub(report[index - 1]).unwrap();
        let abs_diff = diff.abs();

        if abs_diff < 1 || abs_diff > 3 {
            return false;
        };

        if order == 0 {
            order = diff / abs_diff
        } else if order != diff / abs_diff {
            return false;
        }
    }
    true
}

fn solution(s: &str) {
    let mut safe_reports = 0;
    for line in s.lines() {
        if is_safe(
            line.split_whitespace()
                .map(|val| val.parse::<i32>().unwrap())
                .collect(),
        ) {
            safe_reports += 1;
        }
    }
    println!("There are {safe_reports} Safe reports!");
}

fn main() {
    let s = fs::read_to_string("levels.txt").unwrap();
    solution(&s);
}
