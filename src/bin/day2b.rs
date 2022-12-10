use std::fs::File;
use std::io::{self, BufRead};


fn main() {
    let file = File::open(&"data/day2.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    // Consumes the iterator, returns an (Optional) String
    let mut score = 0;
    for result in lines {
        let line = result.unwrap();
        let mut split = line.split(" ");
        let a = split.next().unwrap();
        let b = split.next().unwrap();
        score += check_score(a,b);
    }
    println!("End Score {}", score);
}

fn check_score(a: &str, b: &str) -> i32 {
    return match a {
        "A" => match b {
            "X" => 3+0,
            "Y" => 1+3,
            "Z" => 2+6,
            _ => panic!()
        },
        "B" => match b {
            "X" => 1+0,
            "Y" => 2+3,
            "Z" => 3+6,
            _ => panic!()
        },
        "C" => match b {
            "X" => 2+0,
            "Y" => 3+3,
            "Z" => 1+6,
            _ => panic!()
        },
        _ => panic!()
    };
}
