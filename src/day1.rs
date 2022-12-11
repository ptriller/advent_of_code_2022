use std::fs::File;
use std::io::{self, BufRead};

pub fn day1work1() -> io::Result<usize> {
    let elves = calc_elves();
    return Ok(elves[0]);
}

pub fn day1work2() -> io::Result<usize> {
    let elves = calc_elves();
    return Ok(elves[0..3].iter().fold(0, |x, y| x + y));
}


fn calc_elves() -> Vec<usize> {
    let file = File::open(&"data/day1.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut calories = 0;
    let mut cal_vec: Vec<usize> = vec!();
    // Consumes the iterator, returns an (Optional) String
    for result in lines {
        let line = result.unwrap();
        if line.trim().is_empty() {
            cal_vec.push(calories);
            calories = 0;
        } else {
            calories += line.parse::<usize>().unwrap();
        }
    }
    cal_vec.sort_by(|a, b| b.cmp(a));
    return cal_vec;
}

#[cfg(test)]
mod tests {
    use crate::day1::{day1work1, day1work2};

    #[test]
    fn test_1() {
        match day1work1() {
            Ok(str) => println!("Day 1 Part 1 Calories: {str}"),
            Err(data) => panic!("Something went wrong: {}", data)
        }
    }

    #[test]
    fn test_2() {
        match day1work2() {
            Ok(str) => println!("Day 1 Part 2 Calories: {str}"),
            Err(data) => panic!("Something went wrong: {}", data)
        }
    }

}