use std::{fs, io};
use std::collections::{HashSet, VecDeque};


fn main() {
    if let Err(data) = work() {
        panic!("Something went wrong: {}", data);
    }
}

fn work() -> io::Result<String> {
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
            println!("Offset: {}", processed);
            return Ok("".to_string());
        }
    }
    return Ok("".to_string());
}

