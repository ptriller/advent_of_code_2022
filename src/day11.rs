use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;

use crate::day11::Operand::{NUM, OLD};
use crate::day11::Operation::{ADD, MULT};
use crate::day11::Test::MODULO;

#[derive(Debug)]
enum Operand {
    OLD,
    NUM(usize),
}

#[derive(Debug)]
enum Operation {
    ADD(Operand, Operand),
    MULT(Operand, Operand),
}

#[derive(Debug)]
enum Test {
    MODULO(usize),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: Test,
    true_target: usize,
    false_target: usize,
    inspect: usize,
}

#[derive(Debug)]
struct Tree {
    monkeys: Vec<Monkey>,
}

impl Monkey {
    fn new<I>(lines: &mut I) -> Self
    where
        I: Iterator<Item = Result<String, io::Error>>,
    {
        return Monkey {
            inspect: 0,
            items: Monkey::parse_items(lines.next().unwrap().unwrap()),
            operation: Monkey::parse_operation(lines.next().unwrap().unwrap()),
            test: Monkey::parse_test(lines.next().unwrap().unwrap()),
            true_target: Monkey::parse_conditions(lines.next().unwrap().unwrap(), 29),
            false_target: Monkey::parse_conditions(lines.next().unwrap().unwrap(), 30),
        };
    }

    fn calc_step<F>(&mut self, mapper: &F) -> Vec<(usize, usize)>
    where
        F: Fn(usize) -> usize,
    {
        let result = self
            .items
            .iter()
            .map(|old| Monkey::apply_oepration(&self.operation, *old))
            .map(mapper)
            .map(|worry| {
                if Monkey::apply_test(&self.test, worry) {
                    return (self.true_target, worry);
                }
                return (self.false_target, worry);
            })
            .collect();
        self.inspect += self.items.len() as usize;
        self.items.truncate(0);
        return result;
    }

    fn catch(&mut self, item: usize) {
        self.items.push(item);
    }

    fn apply_test(test: &Test, worry: usize) -> bool {
        let MODULO(val) = test;
        return worry % val == 0;
    }

    fn apply_oepration(op: &Operation, old: usize) -> usize {
        match op {
            ADD(l, r) => Monkey::apply_operand(l, old) + Monkey::apply_operand(r, old),
            MULT(l, r) => Monkey::apply_operand(l, old) * Monkey::apply_operand(r, old),
        }
    }

    fn apply_operand(op: &Operand, old: usize) -> usize {
        match op {
            OLD => old,
            NUM(val) => *val,
        }
    }

    fn parse_items(line: String) -> Vec<usize> {
        Vec::from_iter(
            line.as_str()[18..]
                .split(",")
                .map(|id| id.trim().parse().unwrap()),
        )
    }

    fn parse_operation(line: String) -> Operation {
        let elements: Vec<&str> = line.as_str()[19..].split(" ").collect();
        match elements[1] {
            "+" => ADD(
                Monkey::parse_operand(elements[0]),
                Monkey::parse_operand(elements[2]),
            ),
            "*" => MULT(
                Monkey::parse_operand(elements[0]),
                Monkey::parse_operand(elements[2]),
            ),
            _ => panic!("Merry Chrismas !"),
        }
    }

    fn parse_operand(op: &str) -> Operand {
        if op == "old" {
            return OLD;
        }
        return NUM(op.parse().unwrap());
    }

    fn parse_test(line: String) -> Test {
        MODULO(line.as_str()[21..].parse().unwrap())
    }

    fn parse_conditions(line: String, offset: usize) -> usize {
        line.as_str()[offset..].parse().unwrap()
    }
}

impl Tree {
    fn new<I>(mut lines: I) -> Self
    where
        I: Iterator<Item = Result<String, io::Error>>,
    {
        let mut tree = Tree { monkeys: vec![] };
        loop {
            let _line = lines.next().unwrap();
            tree.monkeys.push(Monkey::new(&mut lines));
            if lines.next().is_none() {
                break;
            }
        }
        return tree;
    }

    fn calc_step<F>(&mut self, mapper: &F)
    where
        F: Fn(usize) -> usize,
    {
        for i in 0..self.monkeys.len() {
            let items = self.monkeys[i].calc_step(mapper);
            for item in items {
                self.monkeys[item.0].catch(item.1);
            }
        }
    }

    fn calc_monkey_business(&self) -> usize {
        let mut monkey: Vec<usize> = self.monkeys.iter().map(|m| m.inspect).collect();
        monkey.sort_by(|a, b| b.cmp(a));
        return monkey[0] * monkey[1];
    }
}

pub fn day11work1() -> io::Result<usize> {
    let file = File::open(&"data/day11.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut tree = Tree::new(&mut lines);
    for _ in 0..20 {
        tree.calc_step(&|x| x / 3);
    }
    return Ok(tree.calc_monkey_business());
}

pub fn day11work2() -> io::Result<usize> {
    let file = File::open(&"data/day11.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut tree = Tree::new(&mut lines);
    let mod_set: HashSet<usize> = HashSet::from_iter(tree.monkeys.iter().map(|m| {
        let MODULO(a) = m.test;
        return a;
    }));
    let kgv = mod_set.iter().fold(1 as usize, |a, b| a * b);
    for _ in 0..10000 {
        tree.calc_step(&|x| x % kgv);
    }
    return Ok(tree.calc_monkey_business());
}

#[cfg(test)]
mod tests {
    use crate::day11::{day11work1, day11work2};

    #[test]
    fn test_1() {
        match day11work1() {
            Ok(num) => println!("Day 11 Part 1 Monkey Business: {num}"),
            Err(data) => panic!("Something went wrong: {}", data),
        }
    }

    #[test]
    fn test_2() {
        match day11work2() {
            Ok(num) => println!("Day 11 Part 2 Monkey Business: {num}"),
            Err(data) => panic!("Something went wrong: {}", data),
        }
    }
}
