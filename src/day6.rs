use std::{fs, io};
use std::collections::{HashSet, VecDeque};

pub fn day6work1() -> io::Result<usize> {
    let data = fs::read_to_string("data/day6.txt")?;
    let mut deq = VecDeque::new();
    let mut processed = 0;
    for c in data.chars() {
        deq.push_back(c);
        processed += 1;
        while deq.len() > 4 {
            deq.pop_front();
        }
        let set: HashSet<char> = HashSet::from_iter(deq.iter().copied());
        if set.len() == 4 {
            return Ok(processed);
        }
    }
    panic!("Noting found !")
}



pub fn day6work2() -> io::Result<usize> {
    let data = fs::read_to_string("data/day6.txt")?;
    let mut deq = VecDeque::new();
    let mut processed = 0;
    for c in data.chars() {
        deq.push_back(c);
        processed += 1;
        while deq.len() > 14 {
            deq.pop_front();
        }
        let set: HashSet<char> = HashSet::from_iter(deq.iter().copied());
        if set.len() == 14 {
            return Ok(processed);
        }
    }
    panic!("Noting found !")
}


#[cfg(test)]
mod tests {
    use crate::day6::{day6work1, day6work2};

    #[test]
    fn test_1() {
        match day6work1() {
            Ok(num) => println!("Day 6 Part 1 Processed: {num}"),
            Err(data) => panic!("Something went wrong: {}", data)
        }
    }

    #[test]
    fn test_2() {
        match day6work2() {
            Ok(num) => println!("Day 6 Part 2 Processed: {num}"),
            Err(data) => panic!("Something went wrong: {}", data)
        }
    }
}