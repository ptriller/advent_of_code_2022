use std::{fs, io};
use std::collections::{HashSet};
use std::io::BufRead;
use crate::day10::Instruction::{ADDX, NOOP};

#[derive(Debug)]
enum Instruction {
    NOOP,
    ADDX(isize),
}

#[derive(Debug)]
struct Computer {
    instructions: Vec<Instruction>,
    pc: usize,
    tick: usize,
    register: isize,
}

impl Computer {
    fn new<I>(iter: I) -> Self
        where
            I: Iterator<Item=Result<String, io::Error>>, {
        return Computer {
            instructions: Vec::from_iter(iter.map(map_instruction)),
            pc: 0,
            tick: 0,
            register: 1,
        };
    }

    fn execute<I>(&mut self, callback: &mut I)
        where
            I: FnMut(usize, isize) -> bool {
        loop {
            match self.instructions[self.pc] {
                NOOP => {
                    self.tick += 1;
                    if !callback(self.tick, self.register) { return; };
                }
                ADDX(val) => {
                    self.tick += 1;
                    if !callback(self.tick, self.register) { return; };
                    self.tick += 1;
                    if !callback(self.tick, self.register) { return; };
                    self.register += val;
                }
            }
            self.pc = (self.pc + 1) % self.instructions.len();
        }
    }
}

fn map_instruction(res: io::Result<String>) -> Instruction {
    let line = res.unwrap();
    if line == "noop" {
        return NOOP;
    } else {
        return ADDX(line.as_str().strip_prefix("addx ").unwrap().parse().unwrap());
    }
}

pub fn day10work1() -> io::Result<isize> {
    let file = fs::File::open(&"data/day10.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut computer = Computer::new(&mut lines);
    let point = HashSet::from([20, 60, 100, 140, 180, 220]);
    let mut sum = 0;
    computer.execute(&mut |tick, register| -> bool {
        if point.contains(&tick) {
            sum += tick as isize * register;
        }
        tick < 220
    });
    return Ok(sum);
}


pub fn day10work2() -> io::Result<String> {
    let file = fs::File::open(&"data/day10.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut computer = Computer::new(&mut lines);
    let mut display = vec![vec![' '; 41]; 6];
    for row in display.iter_mut() {
        row[40] = '\n';
    }
    computer.execute(&mut |tick, register| -> bool {
        let row = (tick - 1) / 40;
        let col = (tick - 1) % 40;
        if (col as isize).abs_diff(register) <= 1 {
            display[row][col] = '#';
        } else {
            display[row][col] = '.';
        }
        tick < 240
    });
    let screen = String::from_iter(display.iter().flat_map(|row| row.iter()));
    return Ok(screen);
}

#[cfg(test)]
mod tests {
    use crate::day10::{day10work1, day10work2};

    #[test]
    fn test_1() {
        match day10work1() {
            Ok(num) => println!("Day 10 Part 1 Signal Strength: {num}"),
            Err(data) => panic!("Something went wrong: {}", data)
        }
    }

    #[test]
    fn test_2() {
        match day10work2() {
            Ok(num) => println!("Day 10 Part 2 Display:\n{num}"),
            Err(data) => panic!("Something went wrong: {}", data)
        }
    }
}