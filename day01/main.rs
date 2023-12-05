use std::io::{BufReader, BufRead};
use std::fs::File;
fn part1(file_path: &str) {
    let f = File::open(file_path).unwrap();
    let reader = BufReader::new(f);
    let mut sum = 0;
    for line in reader.lines().map(|x| x.unwrap()) {
        if let Some(i1) = line.find(|x: char| x.is_numeric()) {
            if let Some(i2) = line.rfind(|x: char| x.is_numeric()) {
                let d1: u32 = line.get(i1..i1+1).unwrap().parse().unwrap();
                let d2: u32 = line.get(i2..i2+1).unwrap().parse().unwrap();
                sum += d1*10 + d2;
            }
        }
    }
    println!("{sum}");
}
fn part2(file_path: &str) {
    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                   "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let f = File::open(file_path).unwrap();
    let reader = BufReader::new(f);
    let mut sum = 0;
    for line in reader.lines().map(|x| x.unwrap()) {
        let (mut i1, mut d1): (usize, usize) = (10000, 0);
        let (mut i2, mut d2): (usize, usize) = (0, 0);
        for (index, token) in digits.iter().enumerate() {
            if let Some(i) = line.rfind(token) {
                if i >= i2 {
                    i2 = i;
                    d2 = if index <= 8 {index+1} else {token.parse().unwrap()}
                }
            }
            if let Some(i) = line.find(token) {
                if i <= i1 {
                    i1 = i;
                    d1 = if index <= 8 {index+1} else {token.parse().unwrap()}
                }
            }
        }
        println!("{} {} {line}", d1, d2);
        sum += d1*10 + d2;
    }
    println!("{sum}");
}
fn main() {
    part1("input.txt");
    part2("input.txt");
}
