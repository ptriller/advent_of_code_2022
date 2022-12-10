use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open(&"data/day1.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut elf = 1;
    let mut calories = 0;
    let mut cal_vec: Vec<i32> = vec!();
    // Consumes the iterator, returns an (Optional) String
    for result in lines {
        let line = result.unwrap();
        if line.trim().is_empty() {
            cal_vec.push(calories);
            println!("Elf {} -  {}", elf, calories);
            calories = 0;
            elf += 1;
        } else {
            calories += line.parse::<i32>().unwrap();
        }
    }
    cal_vec.sort_by(|a, b| b.cmp(a));
    println!("Winning Elves {}, {}, {} = {}", cal_vec[0], cal_vec[1], cal_vec[2], cal_vec[0] + cal_vec[1] + cal_vec[2]);
    println!("Last Elf {}", elf);
}
