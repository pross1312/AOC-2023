use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;

fn check(springs: &Vec<char>, mut start: usize, mut count: i128) -> bool {
    while count > 0 {
        if start >= springs.len() || springs[start] == '.' {
            return false;
        }
        start += 1;
        count -= 1;
    }
    if start < springs.len() {
        return springs[start] == '?' || springs[start] == '.';
    }
    true
}
fn solve(springs: &Vec<char>, spr_groups: &Vec<i128>, cache: &mut HashMap<String, u128>) -> u128 {
    let mut marks: Vec<usize> = Vec::new();
    let mut str: String = springs.iter().collect();
    str += &spr_groups.len().to_string();
    if let Some(x) = cache.get(&str) {
        return *x;
    }
    if spr_groups.len() == 0 {
        if springs.iter().any(|&x| x == '#') {
            return 0;
        } else {
            return 1;
        }
    }
    for i in 0..springs.len() {
        if springs[i] == '?' {
            marks.push(i);
        } else if springs[i] == '#' {
            marks.push(i);
            break;
        }
    }
    let mut sum = 0;
    for i in 0..marks.len() {
        let mark = marks[marks.len()-i-1];
        let count = spr_groups[0];
        if check(springs, mark, count) {
            let start = if spr_groups.len() == 1 {0} else {1} + mark + count as usize;
            sum += solve(&springs[start.min(springs.len())..].to_vec(),
                         &spr_groups[1..].to_vec(),
                         cache);
        }
    }
    cache.insert(str, sum);
    sum
}
fn part1(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let lines: Vec<_> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    let springs: Vec<Vec<char>> = lines.iter().map(|x| x.split(" ").nth(0).unwrap().chars().collect()).collect();
    let spr_groups: Vec<Vec<i128>> = lines.iter().map(|x| x
                                        .split(" ")
                                        .nth(1).unwrap()
                                        .split(",")
                                        .map(|y| y.parse().unwrap())
                                        .collect()).collect();
    let mut ans: u128 = 0;
    for i in 0..springs.len() {
        ans += solve(&springs[i], &spr_groups[i], &mut HashMap::new());
    }
    println!("{ans}");
}
fn part2(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let lines: Vec<_> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    let springs: Vec<&str> = lines.iter().map(|x| x.split(" ").nth(0).unwrap()).collect();
    let springs: Vec<Vec<char>> = springs.iter().map(|&x| vec![x; 1].repeat(5).join("?").chars().collect()).collect();
    let spr_groups: Vec<Vec<i128>> = lines.iter().map(|x| x
                                        .split(" ")
                                        .nth(1).unwrap()
                                        .split(",")
                                        .map(|y| y.parse().unwrap())
                                        .collect::<Vec<_>>().repeat(5)).collect();
    let mut ans: u128 = 0;
    for i in 0..springs.len() {
        ans += solve(&springs[i], &spr_groups[i], &mut HashMap::new());
    }
    println!("{ans}");
}

fn main() {
    part1("input.txt");
    part2("input.txt");
    println!("Hello world");
}
