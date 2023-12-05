use std::io::{BufReader, BufRead};
use std::fs::File;
fn part1(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    for line in reader.lines().map(|x| x.unwrap()) {
        let tokens: Vec<&str> = line.split(" ").collect();
        let game: u32 = tokens[1][..tokens[1].len()-1].parse().unwrap();
        let mut bags: [u32; 3] = [0, 0, 0];
        for i in 0..(tokens.len()-2)/2 {
            if let Ok(n) = tokens[(i+1)*2].parse::<u32>() {
                let tmp: Vec<&str> = tokens[(i+1)*2+1].split(|x| x == ',' || x == ';').collect();
                if tmp[0] == "blue" {
                    bags[0] = std::cmp::max(bags[0], n);
                } else if tmp[0] == "green" {
                    bags[1] = std::cmp::max(bags[1], n);
                } else {
                    bags[2] = std::cmp::max(bags[2], n);
                } 
            }
        }
        // 12 red cubes, 13 green cubes, and 14 blue cubes
        if bags[2] <= 12 && bags[1] <= 13 && bags[0] <= 14 {
            sum += game;
        }
    }
    println!("{sum}");
}
fn part2(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut sum: u32 = 0;
    for line in reader.lines().map(|x| x.unwrap()) {
        let tokens: Vec<&str> = line.split(" ").collect();
        let mut bags: [u32; 3] = [0, 0, 0];
        for i in 0..(tokens.len()-2)/2 {
            if let Ok(n) = tokens[(i+1)*2].parse::<u32>() {
                let tmp: Vec<&str> = tokens[(i+1)*2+1].split(|x| x == ',' || x == ';').collect();
                if tmp[0] == "blue" {
                    bags[0] = std::cmp::max(bags[0], n);
                } else if tmp[0] == "green" {
                    bags[1] = std::cmp::max(bags[1], n);
                } else {
                    bags[2] = std::cmp::max(bags[2], n);
                } 
            }
        }
        sum += bags[0] * bags[1] * bags[2];
    }
    println!("{sum}");
}
fn main() {
    part1("input.txt");
    part2("input.txt");
}
