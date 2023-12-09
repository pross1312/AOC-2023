use std::io::{BufReader, BufRead};
use std::fs::File;

fn part1(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let lines: Vec<_> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    let mut input: Vec<Vec<i32>> = lines.iter()
        .map(|x| x.split(" ").map(|y| y.parse().unwrap()).collect())
        .collect();
    let mut acc = 0;
    for i in 0..input.len() {
        let mut len = input[i].len();
        let mut ans = input[i][len-1];
        while input[i].iter().any(|&x| x != 0) {
            for j in 0..len-1 {
                input[i][j] = input[i][j+1] - input[i][j];
            }
            ans += input[i][len-2];
            input[i].resize(len-1, 0);
            len -= 1;
        }
        acc += ans;
    }
    println!("{acc}");
}
fn part2(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let lines: Vec<_> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    let mut input: Vec<Vec<i32>> = lines.iter()
        .map(|x| x.split(" ").map(|y| y.parse().unwrap()).collect())
        .collect();
    let mut acc = 0;
    for i in 0..input.len() {
        let mut len = input[i].len();
        let mut ans = input[i][0];
        let mut mul = -1;
        while input[i].iter().any(|&x| x != 0) {
            for j in 0..len-1 {
                input[i][j] = input[i][j+1] - input[i][j];
            }
            ans += mul * input[i][0];
            mul = (-1) * mul;
            input[i].resize(len-1, 0);
            len -= 1;
        }
        acc += ans;
    }
    println!("{acc}");
}

fn main() {
    part1("input.txt");
    part2("input.txt");
}
