use std::io::{BufReader, BufRead};
use std::fs::File;
fn part1(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let input: Vec<_> = BufReader::new(file)
        .lines().map(|x| x.unwrap()
                          .split(":").nth(1).unwrap().trim()
                          .split(" ").filter(|&y| y != "")
                          .map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>()).collect();
    let mut answer: i64 = 1;
    for i in 0..input[0].len() {
        let x = input[0][i] as f64;
        let y = input[1][i] as f64;
        assert!((x*x-4.0*y).sqrt() > 0.0);
        let b1 = (x - (x*x-4.0*y).sqrt())/2.0;
        let b2 = (x + (x*x-4.0*y).sqrt())/2.0;
        let b1 = if b1 - b1.ceil() < 1e-5 { (b1+1.0) as i64 } else {b2.ceil() as i64};
        let b2 = if b2 - b2.floor() < 1e-5 { (b2-1.0) as i64 } else {b2.floor() as i64};
        answer *= (b2-b1)+1;
    }
    println!("{}", answer);
}
fn part2(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let input: Vec<_> = BufReader::new(file)
        .lines().map(|x| x.unwrap()
                          .split(":").nth(1).unwrap().trim()
                          .split(" ").filter(|&y| y != "")
                          .fold(String::new(), |acc, y| acc + y)
                          .parse::<i64>().unwrap()).collect();
    let x = input[0] as f64;
    let y = input[1] as f64;
    assert!((x*x-4.0*y).sqrt() > 0.0);
    let b1 = (x - (x*x-4.0*y).sqrt())/2.0;
    let b2 = (x + (x*x-4.0*y).sqrt())/2.0;
    let b1 = if b1 - b1.ceil() < 1e-5 { (b1+1.0) as i64 } else {b2.ceil() as i64};
    let b2 = if b2 - b2.floor() < 1e-5 { (b2-1.0) as i64 } else {b2.floor() as i64};
    println!("{}", (b2-b1)+1);
}

fn main() {
    // b^2 - bx + y < 0;
    // x^2-4y
    part1("input.txt");
    part2("input.txt");
}
