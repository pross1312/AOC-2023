use std::io::{BufReader, BufRead};
use std::fs::File;

fn part1(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut sum: i32 = 0;
    for line in reader.lines().map(|x| x.unwrap()) {
        let input = line.split(":").collect::<Vec<&str>>()[1];
        let [win_cards, cards] = &input
            .split("|")
            .map(|x| x.trim()
                      .split(" ")
                      .filter(|x| x.trim() != "")
                      .map(|x| x.parse::<i32>().unwrap())
                      .collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>()[0..2] else { panic!() };
        let count = cards.iter().filter(|x| win_cards.iter().find(|y| y == x).is_some()).count();
        if count > 0 {
            let two: i32 = 2;
            sum += two.pow(count as u32-1);
        }
    }
    println!("{}", sum);
}
fn part2(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    let mut counts = vec![1; lines.len()];
    for (index, line) in lines.iter().enumerate() {
        let input = line.split(":").collect::<Vec<&str>>()[1];
        let [win_cards, cards] = &input
            .split("|")
            .map(|x| x.trim()
                      .split(" ")
                      .filter(|x| x.trim() != "")
                      .map(|x| x.parse::<i32>().unwrap())
                      .collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>()[0..2] else { panic!() };
        let mut count = cards.iter().filter(|x| win_cards.iter().find(|y| y == x).is_some()).count() as i32;
        while count > 0 {
            counts[index + count as usize] += counts[index];
            count -= 1;
        }
    }
    println!("{:?}", counts.iter().fold(0, |acc, x| acc + x));
}


fn main() {
    part1("input.txt");
    part2("input.txt");
}
