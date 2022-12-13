use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::usize;

struct Wares {
    crates: Vec<Vec<char>>,
}

impl Wares {
    fn new() -> Self {
        return Wares {
            crates: vec![
                vec![],
                vec!['Q', 'M', 'G', 'C', 'L'],
                vec!['R', 'D', 'L', 'C', 'T', 'F', 'H', 'G'],
                vec!['V', 'J', 'F', 'N', 'M', 'T', 'W', 'R'],
                vec!['J', 'F', 'D', 'V', 'Q', 'P'],
                vec!['N', 'F', 'M', 'S', 'L', 'B', 'T'],
                vec!['R', 'N', 'V', 'H', 'C', 'D', 'P'],
                vec!['H', 'C', 'T'],
                vec!['C', 'S', 'J', 'V', 'Z', 'N', 'H', 'P'],
                vec!['Z', 'F', 'H', 'G'],
            ],
        };
    }

    fn process<I, F>(&mut self, iter: &mut I, moveer: F)
    where
        I: Iterator<Item = Result<String, io::Error>>,
        F: Fn(&mut Vec<Vec<char>>, usize, usize, usize) -> (),
    {
        let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)").unwrap();
        for result in iter {
            let line = result.unwrap();
            let cap = re.captures(line.as_str()).unwrap();
            let count = usize::from_str(cap.get(1).unwrap().as_str()).unwrap();
            let from = usize::from_str(cap.get(2).unwrap().as_str()).unwrap();
            let to = usize::from_str(cap.get(3).unwrap().as_str()).unwrap();
            moveer(&mut self.crates, from, to, count);
        }
    }

    fn result(&self) -> String {
        let mut result = vec![];
        for i in 1..10 {
            result.push(self.crates[i].last().unwrap());
        }
        return String::from_iter(result);
    }
}

pub fn day5work1() -> io::Result<String> {
    let mut wares = Wares::new();
    let file = File::open(&"data/day5.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    wares.process(&mut lines, move_9000);
    return Ok(wares.result());
}

pub fn day5work2() -> io::Result<String> {
    let mut wares = Wares::new();
    let file = File::open(&"data/day5.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    wares.process(&mut lines, move_9001);
    return Ok(wares.result());
}

fn move_9000(crates: &mut Vec<Vec<char>>, from: usize, to: usize, count: usize) {
    for _ in 0..count {
        let c = crates[from].pop().unwrap();
        crates[to].push(c);
    }
}

fn move_9001(crates: &mut Vec<Vec<char>>, from: usize, to: usize, count: usize) {
    let newlen = crates[from].len() - count;
    let mut moved = crates[from].as_slice()[newlen..].to_vec();
    crates[from].truncate(newlen);
    crates[to].append(&mut moved);
}

#[cfg(test)]
mod tests {
    use crate::day5::{day5work1, day5work2};

    #[test]
    fn test_1() {
        match day5work1() {
            Ok(str) => println!("Day 5 Part 1 Processed: {str}"),
            Err(data) => panic!("Something went wrong: {}", data),
        }
    }

    #[test]
    fn test_2() {
        match day5work2() {
            Ok(str) => println!("Day 5 Part 2 Processed: {str}"),
            Err(data) => panic!("Something went wrong: {}", data),
        }
    }
}
