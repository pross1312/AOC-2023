use std::io::{BufReader, BufRead};
use std::fs::File;

fn part1(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);
    let mut seeds = String::new();
    reader.read_line(&mut seeds).unwrap();
    let mut seeds: Vec<i64> = seeds
        .split(":")
        .map(|x| x.trim())
        .nth(1).unwrap()
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let mut after_transformed = seeds.clone();
    for line in reader.lines().map(|x| x.unwrap()).filter(|ref x| x.trim() != "") {
        if !line.contains("map") {
            let parsed: Vec<i64> = line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
            for i in 0..seeds.len() {
                if seeds[i] >= parsed[1] && seeds[i] < parsed[1] + parsed[2] {
                    after_transformed[i] = parsed[0] + (seeds[i] - parsed[1])
                }
            }
        } else {
            seeds = after_transformed.clone();
        }
    }
    println!("{}", after_transformed.iter().min().unwrap());
}
fn part2(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);
    let mut seeds = String::new();
    reader.read_line(&mut seeds).unwrap();
    let mut seeds: Vec<i64> = seeds
        .split(":")
        .map(|x| x.trim())
        .nth(1).unwrap()
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let mut after_transformed = seeds.clone();
    for line in reader.lines().map(|x| x.unwrap()).filter(|ref x| x.trim() != "") {
        if !line.contains("map") {
            let parsed: Vec<i64> = line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
            for i in 0..seeds.len()/2 {
                if seeds[2*i+1] == 0 { continue; }
                if seeds[2*i] >= parsed[1] && seeds[2*i] < parsed[1] + parsed[2] {
                    after_transformed[2*i] = seeds[2*i] - parsed[1] + parsed[0];
                    if seeds[2*i] + seeds[2*i+1] - 1 >= parsed[1] + parsed[2] {
                        after_transformed[2*i+1] = parsed[1] + parsed[2] - seeds[2*i];
                        after_transformed.push(parsed[1] + parsed[2]);
                        after_transformed.push(seeds[2*i+1] - after_transformed[2*i+1]);
                    }
                } else if seeds[2*i] < parsed[1] && seeds[2*i] + seeds[2*i+1] - 1 >= parsed[1] {
                    after_transformed[2*i+1] = parsed[1] - seeds[2*i];
                    after_transformed.push(parsed[0]);
                    if seeds[2*i] + seeds[2*i+1] - 1 < parsed[1] + parsed[2] {
                        after_transformed.push(seeds[2*i+1] - after_transformed[2*i+1]);
                    } else {
                        after_transformed.push(parsed[2]);
                        after_transformed.push(parsed[1] + parsed[2]);
                        after_transformed.push(seeds[2*i+1] - after_transformed[2*i+1] - parsed[2]);
                    }
                }
            }
        } else {
            seeds = after_transformed.clone();
        }
    }
    println!("{}", after_transformed.iter().step_by(2).min().unwrap());
}

fn main() {
    part1("input.txt");
    part2("input.txt");
}
