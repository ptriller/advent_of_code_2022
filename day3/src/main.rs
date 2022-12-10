use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open(&"day3/input.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut prio = 0;
    loop {
        let l = lines.next();
        if l.is_none() {
            break;
        }
        let mut set: HashSet<u32> = HashSet::from_iter(l.unwrap().unwrap().chars().map(priority));
        for _ in 2..4 {
            set = set.intersection(&HashSet::from_iter(
                lines.next().unwrap().unwrap().chars().map(priority))
            ).copied().collect();
        }
        assert_eq!(set.len(), 1);
        prio += set.iter().next().unwrap();
    }

    println!("Result: {}", prio)
}


fn main1() {
    let file = File::open(&"day3/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut prio = 0;
    for result in lines {
        let line = result.unwrap();
        let slice = line.len() / 2;
        let left: HashSet<u32> = HashSet::from_iter(line[..slice].chars().map(priority));
        let right = HashSet::from_iter(line[slice..].chars().map(priority));
        for x in left.intersection(&right) {
            prio += x
        }
    }
    println!("Result: {}", prio)
}


fn priority(c: char) -> u32 {
    if c.is_lowercase() {
        return c as u32 - 'a' as u32 + 1;
    }
    let val = c as u32 - 'A' as u32 + 27;
    return val;
}