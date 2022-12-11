use std::collections::HashSet;
use std::{fs, io};
use std::io::BufRead;

type Position = (i32, i32);

struct RopeSpace {
    trail: HashSet<Position>,
    head: Position,
    tail: Position,
}

impl RopeSpace {
    fn new() -> Self {
        RopeSpace {
            trail: HashSet::new(),
            head: (0, 0),
            tail: (0, 0),
        }
    }

    fn drag_rope<I>(&mut self, iter: &mut I)
        where
            I: Iterator<Item=Result<String, io::Error>>, {
        self.trail.insert(self.tail.clone());
        for res in iter {
            let line = res.unwrap();
            let num: usize = line.get(2..).unwrap().parse().unwrap();
            let mv = match line.chars().nth(0).unwrap() {
                'R' => (1, 0),
                'L' => (-1, 0),
                'D' => (0, 1),
                'U' => (0, -1),
                _ => panic!("Illegal Input")
            };
            for _ in 0..num {
                self.head.0 += mv.0;
                self.head.1 += mv.1;
                self.move_tail();
                self.trail.insert(self.tail.clone());
            }
        }
    }
    fn move_tail(&mut self) {
        if (self.head.1 - self.tail.1).abs() > 1 {
            self.tail.0 = self.head.0;
            self.tail.1 = self.head.1 + (self.head.1 - self.tail.1).signum();
        } else if self.head.1 == self.tail.1 && (self.head.0 - self.tail.0).abs() > 1 {
            self.tail.1 = self.head.1;
            self.tail.0 = self.head.0 + (self.head.0 - self.tail.0).signum();
        }
    }
}

pub fn day9work1() -> io::Result<usize> {
    let file = fs::File::open(&"data/day9.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut rope = RopeSpace::new();
    rope.drag_rope(&mut lines);
    return Ok(rope.trail.len());
}