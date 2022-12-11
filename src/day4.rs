use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use regex::Regex;

fn process<F>(consumer: &mut F)
    where
        F: FnMut(usize, usize, usize, usize) {
    let file = File::open(&"data/day4.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    for result in lines {
        let line = result.unwrap();
        let cap = re.captures(line.as_str()).unwrap();
        let a1 = usize::from_str(cap.get(1).unwrap().as_str()).unwrap();
        let a2 = usize::from_str(cap.get(2).unwrap().as_str()).unwrap();
        let b1 = usize::from_str(cap.get(3).unwrap().as_str()).unwrap();
        let b2 = usize::from_str(cap.get(4).unwrap().as_str()).unwrap();
        consumer(a1, a2, b1, b2);
    }
}

pub fn day4work1() -> io::Result<usize> {
    let mut sum = 0;
    process(&mut |a1, a2, b1, b2|
        if (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2) {
            sum += 1;
        });
    return Ok(sum);
}

pub fn day4work2() -> io::Result<usize> {
    let mut sum = 0;
    process(&mut |a1, a2, b1, b2|
        if (a1 >= b1 && a1 <= b2) || (a2 >= b1 && a2 <= b2) ||
            (b1 >= a1 && b1 <= a2) || (b2 >= a1 && b2 <= a2) {
            sum += 1;
        });
    return Ok(sum);
}


#[cfg(test)]
mod tests {
    use crate::day4::{day4work1, day4work2};

    #[test]
    fn test_1() {
        match day4work1() {
            Ok(num) => println!("Day 4 Part 1 Sum: {num}"),
            Err(data) => panic!("Something went wrong: {}", data)
        }
    }

    #[test]
    fn test_2() {
        match day4work2() {
            Ok(num) => println!("Day 4 Part 2 Sum: {num}"),
            Err(data) => panic!("Something went wrong: {}", data)
        }
    }
}