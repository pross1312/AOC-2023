use std::io::{BufReader, BufRead};
use std::fs::File;
fn count_range(mut start: usize, mut end: usize, sorted_range: &Vec<usize>) -> u128 {
    if start > end {
        let tmp = start;
        start = end;
        end = tmp;
    }
    let mut count = 0;
    if end - start <= 1 {
        return 0;
    }
    for &i in sorted_range {
        if i > end {
            break;
        } else if i > start {
            count += 1;
        }
    }
    return count;
}
fn solve(file_path: &str, expand: u128) {
    let file = File::open(file_path).unwrap();
    let lines: Vec<_> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    let mut rows = vec![0usize; 0];
    let mut cols = vec![0usize; 0];
    let mut positions = vec![(0, 0); 0];
    for r in 0..lines.len() {
        if !lines[r].contains('#') {
            rows.push(r);
        }
    }
    lines.iter().enumerate().for_each(|(r, x)| {
        let mut tmp = &x[..];
        let mut acc = 0;
        while let Some(c) = tmp.find('#') {
            positions.push((r, c + acc));
            acc += c+1;
            tmp = &tmp[c+1..];
        }
    });
    for c in 0..lines[0].len() {
        let mut found = false;
        for r in 0..lines.len() {
            if lines[r].chars().nth(c).unwrap() == '#' {
                found = true;
                break;
            }
        }
        if !found {
            cols.push(c);
        }
    }
    rows.sort();
    cols.sort();
    let mut sum = 0u128;
    for i in 0..positions.len() {
        for j in i+1..positions.len() {
            let ((r1, c1), (r2, c2)) = (positions[i], positions[j]);
            let r_dif = if r1 < r2 { r2 - r1 } else { r1 - r2 } as u128;
            let c_dif = if c1 < c2 { c2 - c1 } else { c1 - c2 } as u128;
            sum += r_dif + (expand-1) * count_range(r1, r2, &rows) + c_dif + (expand-1) * count_range(c1, c2, &cols);
        }
    }
    println!("{}", sum);
}

fn main() {
    solve("input.txt", 2);
    solve("input.txt", 1000000);
}
