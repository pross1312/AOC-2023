use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::fs::File;
fn tokenize(lines: &Vec<String>) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let inst: Vec<_> = lines[0].chars().collect();
    let mut maps: HashMap<&str, (&str, &str)> = HashMap::new();
    lines[2..].iter().map(|x| x.split(" = ")
                  .collect::<Vec<_>>())
         .for_each(|x| {
             let route  = x[1].split(", ")
                 .map(|tok| tok.trim_matches(|x| x == '(' || x == ')'))
                 .collect::<Vec<_>>();
             maps.insert(x[0], (route[0], route[1]));
         });
    return (inst, maps);
}
fn find_first<'a, FN>(mut start: &'a str, inst: &Vec<char>,
                        cond: FN, maps: &HashMap<&'a str, (&'a str, &'a str)>) -> (usize, &'a str)
where FN: Fn(&str) -> bool {
    let mut count = 0;
    loop {
        start = if inst[count%inst.len()] == 'L' {
            &maps[start].0
        } else {
            &maps[start].1
        };
        count += 1;
        if cond(start) { break; }
    }
    return (count, start);
}
fn part1(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let lines: Vec<_> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    let (inst, maps) = tokenize(&lines);
    let (count, _) = find_first("AAA", &inst, |x| x == "ZZZ", &maps);
    println!("{count}");
}
fn lcm(a: usize, b: usize) -> usize { 
    let greater = a.max(b); 
    let smallest = a.min(b); 
    let mut i = greater;
    loop {
        if i % smallest  == 0 {
            return i;
        }
        i += greater;
    }
} 
fn part2(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let lines: Vec<_> = BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    let (inst, maps) = tokenize(&lines);
    let cur: Vec<_> = maps.keys().filter(|x| x.ends_with("A")).map(|x| *x).collect();
    let condition = |x: &str| x.ends_with("Z");
    // -_- some how i found the pattern, a hack to work around the question i think
    // kinda disappointed
    let counts: Vec<_> = cur.iter().map(|x| {
        let (count, _) = find_first(x, &inst, condition, &maps);
        return count/inst.len();
    }).collect();
    let mut ans = counts[0];
    for i in 1..counts.len() {
        ans = lcm(ans, counts[i]);
    }
    println!("{}", ans as i128 * inst.len() as i128);
}
fn main() {
    part1("input.txt");
    part2("input.txt");
}
