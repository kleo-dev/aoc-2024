use std::fs;

/// it 'appends' to a Vec and acts like a sorting 'algo'
fn algo_append(v: &mut Vec<i32>, i: i32) {
    if let Some(pos) = v.iter().position(|&x| x > i) {
        v.insert(pos, i);
    } else {
        v.push(i);
    }
}

fn solution(s: &str) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    // let mut disc: Vec<u32> = Vec::new();
    let mut total = 0;
    for id_line in s.split('\n') {
        let id = id_line.split("   ").collect::<Vec<&str>>();
        let l_dis = id[0].parse::<i32>().unwrap();
        let r_dis = id[1].parse::<i32>().unwrap();
        algo_append(&mut left, l_dis);
        algo_append(&mut right, r_dis);
    }

    for i in 0..left.len() {
        total += left[i].abs_diff(right[i]);
    }

    println!("{total}")
}

fn main() {
    let s = fs::read_to_string("locs.txt").unwrap();

    solution(&s);
}
