use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};


pub fn day3work1() -> io::Result<usize> {
    let file = File::open(&"data/day3.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut prio = 0;
    for result in lines {
        let line = result.unwrap();
        let slice = line.len() / 2;
        let left: HashSet<usize> = HashSet::from_iter(line[..slice].chars().map(priority));
        let right = HashSet::from_iter(line[slice..].chars().map(priority));
        for x in left.intersection(&right) {
            prio += x
        }
    }
    return Ok(prio);
}


pub fn day3work2() -> io::Result<usize> {
    let file = File::open(&"data/day3.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut prio = 0;
    loop {
        let l = lines.next();
        if l.is_none() {
            break;
        }
        let mut set: HashSet<usize> = HashSet::from_iter(l.unwrap().unwrap().chars().map(priority));
        for _ in 2..4 {
            set = set.intersection(&HashSet::from_iter(
                lines.next().unwrap().unwrap().chars().map(priority))
            ).copied().collect();
        }
        assert_eq!(set.len(), 1);
        prio += set.iter().next().unwrap();
    }
    return Ok(prio);
}


fn priority(c: char) -> usize {
    if c.is_lowercase() {
        return c as usize - 'a' as usize + 1;
    }
    let val = c as usize - 'A' as usize + 27;
    return val;
}

#[cfg(test)]
mod tests {
    use crate::day3::{day3work1, day3work2};

    #[test]
    fn test_1() {
        match day3work1() {
            Ok(num) => println!("Day 3 Part 1 Priority: {num}"),
            Err(data) => panic!("Something went wrong: {}", data)
        }
    }

    #[test]
    fn test_2() {
        match day3work2() {
            Ok(num) => println!("Day 3 Part 2 Priority: {num}"),
            Err(data) => panic!("Something went wrong: {}", data)
        }
    }
}