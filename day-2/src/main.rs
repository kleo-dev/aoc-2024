use std::fs;

fn is_safe(report: Vec<i32>, problem_dampener: bool) -> bool {
    let mut order = 0;
    let mut safe = true;

    for index in 1..report.len() {
        let diff = report[index].checked_sub(report[index - 1]).unwrap();
        let abs_diff = diff.abs();

        if abs_diff < 1 || abs_diff > 3 {
            safe = false;
        };

        if order == 0 && abs_diff > 0 {
            order = diff / abs_diff
        } else if diff != 0 && order != diff / abs_diff {
            safe = false;
        }
    }

    if !safe && problem_dampener {
        for index in 0..report.len() {
            let mut modified = report.clone();
            modified.remove(index);
            if is_safe(modified, false) {
                return true;
            }
        }
        false
    } else {
        safe
    }
}

fn solution(s: &str) {
    let mut safe_reports = 0;
    for line in s.lines() {
        if is_safe(
            line.split_whitespace()
                .map(|val| val.parse::<i32>().unwrap())
                .collect(),
            true,
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
