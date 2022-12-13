use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn calculate_score(score_map: &HashMap<char, HashMap<char, usize>>) -> io::Result<usize> {
    let file = File::open(&"data/day2.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    // Consumes the iterator, returns an (Optional) String
    let mut score = 0;
    for result in lines {
        let line = result.unwrap();
        let a = line.chars().nth(0).unwrap().clone();
        let b = line.chars().nth(2).unwrap().clone();
        score += score_map.get(&a).unwrap().get(&b).unwrap();
    }
    return Ok(score);
}

pub fn day2work1() -> io::Result<usize> {
    let score_map = HashMap::from([
        (
            'A',
            HashMap::from([('X', 1 + 3), ('Y', 2 + 6), ('Z', 3 + 0)]),
        ),
        (
            'B',
            HashMap::from([('X', 1 + 0), ('Y', 2 + 3), ('Z', 3 + 6)]),
        ),
        (
            'C',
            HashMap::from([('X', 1 + 6), ('Y', 2 + 0), ('Z', 3 + 3)]),
        ),
    ]);
    return calculate_score(&score_map);
}

pub fn day2work2() -> io::Result<usize> {
    let score_map = HashMap::from([
        (
            'A',
            HashMap::from([('X', 3 + 0), ('Y', 1 + 3), ('Z', 2 + 6)]),
        ),
        (
            'B',
            HashMap::from([('X', 1 + 0), ('Y', 2 + 3), ('Z', 3 + 6)]),
        ),
        (
            'C',
            HashMap::from([('X', 2 + 0), ('Y', 3 + 3), ('Z', 1 + 6)]),
        ),
    ]);
    return calculate_score(&score_map);
}

#[cfg(test)]
mod tests {
    use crate::day2::{day2work1, day2work2};

    #[test]
    fn test_1() {
        match day2work1() {
            Ok(num) => println!("Day 2 Part 1 Score: {num}"),
            Err(data) => panic!("Something went wrong: {}", data),
        }
    }

    #[test]
    fn test_2() {
        match day2work2() {
            Ok(num) => println!("Day 2 Part 2 Score: {num}"),
            Err(data) => panic!("Something went wrong: {}", data),
        }
    }
}
