use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use regex::Regex;

fn main() {
    let file = File::open(&"day4/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut sum = 0;
    let mut sum2 = 0;
    for result in lines {
        let line = result.unwrap();
        let cap = re.captures(line.as_str()).unwrap();
        let a1 = i32::from_str(cap.get(1).unwrap().as_str()).unwrap();
        let a2 = i32::from_str(cap.get(2).unwrap().as_str()).unwrap();
        let b1 = i32::from_str(cap.get(3).unwrap().as_str()).unwrap();
        let b2 = i32::from_str(cap.get(4).unwrap().as_str()).unwrap();
        if (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2) {
            sum += 1;
        }
        if (a1 >= b1 && a1 <= b2) || (a2 >= b1 && a2 <= b2) ||
            (b1 >= a1 && b1 <= a2) || (b2 >= a1 && b2 <= a2) {
            sum2 += 1;
        }
    }
    println!("Result {}", sum);
    println!("Result2 {}", sum2);
}

