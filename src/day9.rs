use std::collections::HashSet;
use std::io::BufRead;
use std::{fs, io};

type Position = (i32, i32);

#[derive(Debug)]
struct RopeSpace {
    trail: HashSet<Position>,
    head: Position,
    tail: Vec<Position>,
}

impl RopeSpace {
    fn new(tails: usize) -> Self {
        RopeSpace {
            trail: HashSet::new(),
            head: (0, 0),
            tail: vec![(0, 0); tails],
        }
    }

    fn drag_rope<I>(&mut self, iter: &mut I)
    where
        I: Iterator<Item = Result<String, io::Error>>,
    {
        self.trail.insert(self.tail.last().unwrap().clone());
        for res in iter {
            let line = res.unwrap();
            let num: usize = line.get(2..).unwrap().parse().unwrap();
            let mv = match line.chars().nth(0).unwrap() {
                'R' => (1, 0),
                'L' => (-1, 0),
                'D' => (0, -1),
                'U' => (0, 1),
                _ => panic!("Illegal Input"),
            };
            for _ in 0..num {
                self.head.0 += mv.0;
                self.head.1 += mv.1;
                move_tail(&self.head, &mut self.tail.first_mut().unwrap());
                for i in 0..self.tail.len() - 1 {
                    let head = self.tail[i].clone();
                    move_tail(&head, &mut self.tail[i + 1]);
                }
                self.trail.insert(self.tail.last().unwrap().clone());
            }
        }
    }
}

fn move_tail(head: &Position, tail: &mut Position) {
    let one = (head.1 - tail.1).abs();
    let zero = (head.0 - tail.0).abs();
    if one > 1 && zero > 1 {
        tail.0 = head.0 - (head.0 - tail.0).signum();
        tail.1 = head.1 - (head.1 - tail.1).signum();
    } else if one > 1 {
        tail.0 = head.0;
        tail.1 = head.1 - (head.1 - tail.1).signum();
    } else if zero > 1 {
        tail.1 = head.1;
        tail.0 = head.0 - (head.0 - tail.0).signum();
    }
}

pub fn day9work1() -> io::Result<usize> {
    let file = fs::File::open(&"data/day9.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut rope = RopeSpace::new(1);
    rope.drag_rope(&mut lines);
    return Ok(rope.trail.len());
}

pub fn day9work2() -> io::Result<usize> {
    let file = fs::File::open(&"data/day9.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut rope = RopeSpace::new(9);
    rope.drag_rope(&mut lines);
    return Ok(rope.trail.len());
}

#[cfg(test)]
mod tests {
    use crate::day9::{day9work1, day9work2};

    #[test]
    fn test_1() {
        match day9work1() {
            Ok(num) => println!("Day 9 Part 1 Positions: {num}"),
            Err(data) => panic!("Something went wrong: {}", data),
        }
    }

    #[test]
    fn test_2() {
        match day9work2() {
            Ok(num) => println!("Day 9 Part 2 Positions: {num}"),
            Err(data) => panic!("Something went wrong: {}", data),
        }
    }
}
