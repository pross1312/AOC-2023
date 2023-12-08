use std::io::{BufReader, BufRead};
use std::fs::File;
use std::cmp::Ordering;

fn get_dif_and_lcs(cards: &mut Vec<char>) -> (i32, i32) {
    cards.sort();
    let mut dif = 0;
    let mut max_same = 0;
    let mut temp_max = if cards.len() != 0 {1} else {0};
    for i in 1..cards.len() {
        if cards[i] != cards[i-1] {
            dif += 1;
            max_same = temp_max.max(max_same);
            temp_max = 1;
        } else {
            temp_max += 1;
        }
    }
    max_same = temp_max.max(max_same);
    (max_same, dif)
}

fn check_rank1(mut cards: Vec<char>) -> i32 {
    assert!(cards.len() == 5);
    let (max_same, dif) = get_dif_and_lcs(&mut cards);
    max_same + (3 - dif)
    // if dif == 0 { return max_same + 3; }
    // else if dif == 1 { return max_same + 2; }
    // else if dif == 2 { return max_same + 1; }
    // else if dif == 3 { return max_same; }
}
fn check_rank2(mut cards: Vec<char>) -> i32 {
    let mut jokers = 0;
    for x in &cards {
        if x == &'J' { jokers += 1; }
    }
    cards.retain(|&x| x != 'J');
    cards.sort();
    let (max_same, dif) = get_dif_and_lcs(&mut cards);
    max_same + jokers + (3 - dif)
}

fn solve<F>(file_path: &str, card_order: &str, mut value_of: F)
where F: FnMut(Vec<char>) -> i32 {
    let file = File::open(file_path).unwrap();
    let mut lines: Vec<Vec<String>>  = BufReader::new(file).lines()
        .map(|x| x.unwrap().split(" ").map(|y| y.to_owned()).collect::<Vec<_>>()).collect();
    lines.sort_by(|a, b| {
        let a: Vec<_> = a[0].chars().collect();
        let b: Vec<_> = b[0].chars().collect();
        let av = value_of(a.clone());
        let bv = value_of(b.clone());
        if av == bv {
            let mut result = Ordering::Equal;
            for i in 0..5 {
                if a[i] != b[i] {
                    result = card_order.find(b[i]).cmp(&card_order.find(a[i]));
                    break;
                }
            }
            result
        } else {
            av.cmp(&bv)
        }
    });
    let mut acc: u64 = 0;
    for (i, x) in lines.iter().enumerate() {
        acc += x[1].parse::<u64>().unwrap()*(i as u64+1);
    }
    println!("{}", acc);
}

fn main() {
    solve("input.txt", "AKQJT98765432", check_rank1);
    solve("input.txt", "AKQT98765432J", check_rank2);
}
