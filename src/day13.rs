use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::Peekable;
use crate::day13::Row::{ARRAY, NUM};
use crate::day13::State::{Dunno, Right, Wrong};

#[derive(Debug, Clone)]
enum Row {
    ARRAY(Vec<Row>),
    NUM(usize),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum State {
    Wrong,
    Right,
    Dunno,
}


impl PartialEq<Self> for Row {
    fn eq(&self, other: &Self) -> bool {
        return self.check_order(other) == Dunno;
    }
}

impl PartialOrd<Self> for Row {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.check_order(other) {
            Right => Less,
            Wrong => Greater,
            Dunno => Equal
        })
    }
}

impl Eq for Row {}

impl Ord for Row {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


impl Row {
    fn check_order(&self, other: &Self) -> State {
        match (self, other) {
            (NUM(left), NUM(right)) if left == right => Dunno,
            (NUM(left), NUM(right)) if left < right => Right,
            (NUM(left), NUM(right)) if left > right => Wrong,
            (NUM(_), NUM(_)) => panic!("Opps all Wrong"),
            (NUM(_), ARRAY(right)) => Row::check_array(&vec![self.clone()], right),
            (ARRAY(left), NUM(_)) => Row::check_array(left, &vec![other.clone()]),
            (ARRAY(left), ARRAY(right)) => Row::check_array(left, right),
        }
    }

    fn check_array(left: &Vec<Row>, right: &Vec<Row>) -> State {
        let mut left_iter = left.iter();
        let mut right_iter = right.iter();
        loop {
            let op_left = left_iter.next();
            let op_right = right_iter.next();
            if op_left.is_none() && op_right.is_none() {
                return Dunno;
            }
            if op_left.is_none() {
                return Right;
            }
            if op_right.is_none() {
                return Wrong;
            }
            match op_left.unwrap().check_order(op_right.unwrap()) {
                Dunno => (),
                d => return d
            }
        }
    }
}

fn parse_line(line: &mut String) -> Row {
    let mut iter = line.chars().peekable();
    parse_row(&mut iter)
}

fn parse_row<I: Iterator<Item=char>>(iter: &mut Peekable<I>) -> Row {
    let pre = iter.peek().unwrap();
    if pre.is_digit(10) {
        return parse_digit(iter);
    } else if *pre == '[' {
        return parse_array(iter);
    }
    panic!("Ooompf {pre}");
}

fn parse_array<I: Iterator<Item=char>>(iter: &mut Peekable<I>) -> Row {
    let mut list = vec![];
    assert_eq!('[', iter.next().unwrap());
    if *iter.peek().unwrap() == ']' {
        iter.next();
        return ARRAY(vec![]);
    }
    loop {
        list.push(parse_row(iter));
        let chr = iter.next().unwrap();
        if chr == ']' {
            break;
        }
        assert_eq!(',', chr);
    }
    ARRAY(list)
}

fn parse_digit<I: Iterator<Item=char>>(iter: &mut Peekable<I>) -> Row
{
    let mut chrs = vec![];
    while iter.peek().unwrap().is_digit(10) {
        chrs.push(iter.next().unwrap());
    }
    let str = String::from_iter(chrs);
    NUM(str.parse().unwrap())
}

pub fn day13work1() -> io::Result<usize> {
    let file = File::open(&"data/day13.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut success = 0;
    let mut idx = 0;
    loop {
        idx += 1;
        let row1 = parse_line(&mut lines.next().unwrap().unwrap());
        let row2 = parse_line(&mut lines.next().unwrap().unwrap());
        if row1.check_order(&row2) == Right {
            success += idx;
        }
        if lines.next().is_none() {
            break;
        }
    }
    return Ok(success);
}

pub fn day13work2() -> io::Result<usize> {
    let file = File::open(&"data/day13.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let marker1 = parse_line(&mut "[[2]]".to_string());
    let marker2 = parse_line(&mut "[[6]]".to_string());
    let mut packet = vec![marker1.clone(), marker2.clone()];
    loop {
        packet.push(parse_line(&mut lines.next().unwrap().unwrap()));
        packet.push(parse_line(&mut lines.next().unwrap().unwrap()));
        if lines.next().is_none() {
            break;
        }
    }
    packet.sort();
    let mut val = 1;
    for idx in 0..packet.len() {
        if packet[idx] == marker1 || packet[idx] == marker2 {
            val *= idx + 1;
        }
    }
    return Ok(val);
}


#[cfg(test)]
mod test {
    use crate::day13::{day13work1, day13work2};

    #[test]
    fn test_1() {
        match day13work1() {
            Ok(num) => println!("Day 13 Part 1 Sum: {num}"),
            Err(data) => panic!("Something went wrong: {}", data),
        }
    }

    #[test]
    fn test_2() {
        match day13work2() {
            Ok(num) => println!("Day 13 Part 2 Sum: {num}"),
            Err(data) => panic!("Something went wrong: {}", data),
        }
    }
}