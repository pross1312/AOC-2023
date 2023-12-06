use std::cmp::{min, max};
use std::io::{BufReader, BufRead};
use std::fs::File;

fn part1(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    let mut sum: i64 = 0;
    for (line_n, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        let mut idx: usize = 0;
        while idx < line.len() {
            while idx < line.len() && !chars[idx].is_numeric() {
                idx += 1;
            }
            let mut end: usize = idx;
            while end < line.len() && chars[end].is_numeric() {
                end += 1;
            }
            let line_n = line_n as i32;
            for l in max(0, line_n-1)..min(lines.len() as i32, line_n+2) {
                let chars: Vec<char> = lines[l as usize].chars().collect();
                for i in max(idx as i32-1, 0)..min(end as i32+1, line.len() as i32) {
                    if !chars[i as usize].is_numeric() && chars[i as usize] != '.' {
                        sum += line[idx..end].parse::<i64>().unwrap();
                    }
                }
            }
            idx = end
        }
    }
    println!("{sum}");
}

fn part2(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    let mut cache: Vec<(i32, i32, i64, i64)> = vec![];
    for (line_n, line) in lines.iter().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        let mut idx: usize = 0;
        while idx < line.len() {
            while idx < line.len() && !chars[idx].is_numeric() {
                idx += 1;
            }
            let mut end: usize = idx;
            while end < line.len() && chars[end].is_numeric() {
                end += 1;
            }
            let line_n = line_n as i32;
            for l in max(0, line_n-1)..min(lines.len() as i32, line_n+2) {
                let chars: Vec<char> = lines[l as usize].chars().collect();
                for i in max(idx as i32-1, 0)..min(end as i32+1, line.len() as i32) {
                    if chars[i as usize] == '*' {
                        let number: i64 = line[idx..end].parse().unwrap();
                        let mut found = false;
                        for x in cache.iter_mut() {
                            if x.0 == l && x.1 == i {
                                x.2 += 1;
                                x.3 *= number;
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            cache.push((l, i, 1, number))
                        }
                    }
                }
            }
            idx = end
        }
    }
    let mut sum: i64 = 0;
    for x in cache {
        if x.2 == 2 {
            sum += x.3
        }
    }
    println!("{}", sum);
}

fn main() {
    part1("input.txt");
    part2("input.txt");
}
