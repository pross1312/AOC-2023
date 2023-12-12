use std::io::{BufReader, BufRead};
use std::fs::File;
const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (-1, -1), (1, -1), (-1, 1)
];
fn is_valid(movement: (i32, i32), cur: (i32, i32), maps: &Vec<Vec<char>>) -> bool {
    let r = cur.0 + movement.0;
    let c = cur.1 + movement.1;
    return if r < 0 || c < 0 {
        false
    } else if r >= maps.len() as i32 || c >= maps[r as usize].len() as i32 {
        false
    } else {
        true
    }
}
fn get_move(cur: (i32, i32), map: &Vec<Vec<char>>, prev: (i32, i32)) -> Option<(i32, i32)> {
    let cur_char = map[cur.0 as usize][cur.1 as usize];
    let movement = match cur_char {
        '|' => if prev.1 == 0 && prev.0 != 0 {
            Some(prev)
        } else { None }
        '-' => if prev.0 == 0 && prev.1 != 0 && is_valid(prev, cur, map) {
            Some(prev)
        } else { None }
        'L' => if (prev.0 == 1 && prev.1 == 0) || (prev.0 == 0 && prev.1 == -1)  {
            Some((prev.1, prev.0))
        } else { None }
        'J' => if (prev.0 == 0 && prev.1 == 1) || (prev.0 == 1 && prev.1 == 0) {
            Some((-prev.1, -prev.0))
        } else { None }
        '7' => if (prev.1 == 1 && prev.0 == 0) || (prev.0 == -1 && prev.1 == 0) {
            Some((prev.1, prev.0))
        } else { None }
        'F' => if (prev.1 == -1 && prev.0 == 0) || (prev.0 == -1 && prev.1 == 0) {
            Some((-prev.1, -prev.0))
        } else { None }
        _ => {
            None
        },
    };
    if let Some(m) = movement {
        if is_valid(m, cur, map) { return movement; }
    }
    None
}
fn traverse(mut movement: (i32, i32), mut cur: (i32, i32), map: &Vec<Vec<char>>) -> Option<u32> {
    let mut count = 1;
    loop {
        cur = ((cur.0 + movement.0), (cur.1 + movement.1));
        if map[cur.0 as usize][cur.1 as usize] == 'S' {
            return Some(count);
        }
        if let Some(next_move) = get_move(cur, map, movement) {
            movement = next_move;
            count += 1;
        } else {
            break;
        }
    }
    None
}
fn part1(file_path: &str) {
    // const inst = ['|', '-', 'L', 'J', '7', 'F'];
    let file = File::open(file_path).unwrap();
    let lines: Vec<Vec<char>> = BufReader::new(file).lines()
        .map(|x| x.unwrap().chars().collect()).collect();
    let mut cur = (0, 0);
    'find: for r in 0..lines.len() {
        for c in 0..lines[r].len() {
            if lines[r][c] == 'S' {
                cur = (r as i32, c as i32);
                break 'find;
            }
        }
    }
    if is_valid((1, 0), cur, &lines) {
        if let Some(x) = traverse((1, 0), cur, &lines) {
            println!("{}", x/2);
        }
    }
    if is_valid((0, 1), cur, &lines) {
        if let Some(x) = traverse((0, 1), cur, &lines) {
            println!("{}", x/2);
        }
    }
    if is_valid((-1, 0), cur, &lines) {
        if let Some(x) = traverse((-1, 0), cur, &lines) {
            println!("{}", x/2);
        }
    }
    if is_valid((0, -1), cur, &lines) {
        if let Some(x) = traverse((0, -1), cur, &lines) {
            println!("{}", x/2);
        }
    }
}
fn traverse2(mut movement: (i32, i32), mut cur: (i32, i32), map: &mut Vec<Vec<char>>) -> Option<Vec<Vec<String>>> {
    let mut new_map = vec![vec!["-1".to_owned(); map[0].len()]; map.len()];
    new_map[cur.0 as usize][cur.1 as usize] = "S".to_owned();
    loop {
        cur = ((cur.0 + movement.0), (cur.1 + movement.1));
        let cur_char = map[cur.0 as usize][cur.1 as usize];
        if cur_char == 'S' {
            return Some(new_map);
        }
        new_map[cur.0 as usize][cur.1 as usize] = ".".to_string();
        if movement.x != 0 {
            let new_map[cur.0-1][cur.1]
            new_map
        }
        if let Some(next_move) = get_move(cur, map, movement) {
            movement = next_move;
        } else {
            break;
        }
    }
    None
}
fn print_map(map: &Vec<Vec<String>>) {
    for line in map {
        for x in line {
            print!("{:<5}", x);
        }
        println!();
    }
}
fn part2(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let mut lines: Vec<Vec<char>> = BufReader::new(file).lines()
        .map(|x| x.unwrap().chars().collect()).collect();
    let mut cur = (0, 0);
    'find: for r in 0..lines.len() {
        for c in 0..lines[r].len() {
            if lines[r][c] == 'S' {
                cur = (r as i32, c as i32);
                break 'find;
            }
        }
    }
    if is_valid((1, 0), cur, &lines) {
        if let Some(new_map) = traverse2((1, 0), cur, &mut lines) {
            print_map(&new_map);
            return;
        }
    }
    if is_valid((0, 1), cur, &lines) {
        if let Some(new_map) = traverse2((0, 1), cur, &mut lines) {
            print_map(&new_map);
            return;
        }
    }
    if is_valid((-1, 0), cur, &lines) {
        if let Some(new_map) = traverse2((-1, 0), cur, &mut lines) {
            print_map(&new_map);
            return;
        }
    }
    if is_valid((0, -1), cur, &lines) {
        if let Some(new_map) = traverse2((0, -1), cur, &mut lines) {
            print_map(&new_map);
            return;
        }
    }
}

fn main() {
    part1("input.txt");
    part2("sample2.txt");
}
