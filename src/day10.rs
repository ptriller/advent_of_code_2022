use std::{fs, io};
use std::io::BufRead;


pub fn day10work1() -> io::Result<usize> {
    let file = fs::File::open(&"data/day10.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    return Ok(1);
}


#[cfg(test)]
mod tests {
    use crate::day10::day10work1;

    #[test]
    fn test_1() {
        match day10work1() {
            Ok(num) => println!("Day 10 Part 1 Signal Strength: {num}"),
            Err(data) => panic!("Something went wrong: {}", data)
        }
    }
}