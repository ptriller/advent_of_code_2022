use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};


fn main() {
    let file = File::open(&"data/day3.txt").unwrap();
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