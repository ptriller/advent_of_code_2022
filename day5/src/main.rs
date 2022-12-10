use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::usize;
use regex::Regex;

fn main() {
    let mut crates = vec!(
        vec!(),
        vec!('Q', 'M', 'G', 'C', 'L'),
        vec!('R', 'D', 'L', 'C', 'T', 'F', 'H', 'G'),
        vec!('V', 'J', 'F', 'N', 'M', 'T', 'W', 'R'),
        vec!('J', 'F', 'D', 'V', 'Q', 'P'),
        vec!('N', 'F', 'M', 'S', 'L', 'B', 'T'),
        vec!('R', 'N', 'V', 'H', 'C', 'D', 'P'),
        vec!('H', 'C', 'T'),
        vec!('C', 'S', 'J', 'V', 'Z', 'N', 'H', 'P'),
        vec!('Z', 'F', 'H', 'G')
    );
    let file = File::open(&"day5/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)").unwrap();
    for result in lines {
        let line = result.unwrap();
        let cap = re.captures(line.as_str()).unwrap();
        let count = usize::from_str(cap.get(1).unwrap().as_str()).unwrap();
        let from = usize::from_str(cap.get(2).unwrap().as_str()).unwrap();
        let to = usize::from_str(cap.get(3).unwrap().as_str()).unwrap();
        let newlen = crates[from].len() - count;
        let mut moved = crates[from].as_slice()[newlen..].to_vec();
        crates[from].resize(newlen, ' ');
        crates[to].append(&mut moved);
    }
    for i in 1..10 {
        print!("{}", crates[i].last().unwrap())
    }
    println!()
}

fn main1() {
    let mut crates = vec!(
        vec!(),
        vec!('Q', 'M', 'G', 'C', 'L'),
        vec!('R', 'D', 'L', 'C', 'T', 'F', 'H', 'G'),
        vec!('V', 'J', 'F', 'N', 'M', 'T', 'W', 'R'),
        vec!('J', 'F', 'D', 'V', 'Q', 'P'),
        vec!('N', 'F', 'M', 'S', 'L', 'B', 'T'),
        vec!('R', 'N', 'V', 'H', 'C', 'D', 'P'),
        vec!('H', 'C', 'T'),
        vec!('C', 'S', 'J', 'V', 'Z', 'N', 'H', 'P'),
        vec!('Z', 'F', 'H', 'G')
    );
    let file = File::open(&"day5/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)").unwrap();
    for result in lines {
        let line = result.unwrap();
        let cap = re.captures(line.as_str()).unwrap();
        let count = usize::from_str(cap.get(1).unwrap().as_str()).unwrap();
        let from = usize::from_str(cap.get(2).unwrap().as_str()).unwrap();
        let to = usize::from_str(cap.get(3).unwrap().as_str()).unwrap();
        for _ in 0..count {
            let c = crates[from].pop().unwrap();
            crates[to].push(c);
        }
    }
    for i in 1..10 {
        print!("{}", crates[i].last().unwrap())
    }
    println!()
}

