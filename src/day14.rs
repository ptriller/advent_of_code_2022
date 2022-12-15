use std::fs::File;
use std::io;
use std::io::BufRead;

struct Cave {
    map: Vec<Vec<char>>,
    left: usize,
    right: usize,
    bottom: usize,
}

impl Cave {
    fn new<I>(iter: &mut I) -> Self
        where
            I: Iterator<Item=Result<String, io::Error>> {
        let mut result = Cave {
            map: vec![vec!['.'; 600]; 600],
            left: 500,
            right: 500,
            bottom: 0,
        };
        for res in iter {
            let line = res.unwrap();
            let mut pos_iter = line.split(" -> ");
            let mut pos = Cave::parse_pos(pos_iter.next().unwrap());
            for goal_str in pos_iter {
                let goal = Cave::parse_pos(goal_str);
                result.left = goal.0.min(result.left);
                result.right = goal.0.max(result.right);
                result.bottom = goal.1.max(result.bottom);
                while pos != goal {
                    result.map[pos.0][pos.1] = '#';
                    pos = ((pos.0 as isize + (goal.0 as isize - pos.0 as isize).signum()) as usize,
                           (pos.1 as isize + (goal.1 as isize - pos.1 as isize).signum()) as usize);
                }
            }
        }
        return result;
    }

    fn add_sand(&mut self) -> bool {
        let mut sand = (500, 0);
        while self.bottom >= sand.1 && self.left <= sand.0 && self.right >= sand.0 {
            if self.map[sand.0][sand.1 + 1] == '.' {
                sand = (sand.0, sand.1 + 1);
            } else if self.map[sand.0 - 1][sand.1 + 1] == '.' {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if self.map[sand.0 + 1][sand.1 + 1] == '.' {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                self.map[sand.0][sand.1] = 'o';
                return true;
            }
        }
        return false;
    }

    fn parse_pos(pair: &str) -> (usize, usize) {
        let mut num = pair.split(",");
        return (num.next().unwrap().parse().unwrap(),
                num.next().unwrap().parse().unwrap());
    }
}

pub fn day14work1() -> io::Result<usize> {
    let file = File::open(&"data/day14.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut cave = Cave::new(&mut lines);
    let mut count = 0;
    while cave.add_sand() {
        count += 11;
    }
    println!("{}:{}", cave.left, cave.right);
    Ok(count)
}


#[cfg(test)]
mod test {
    use crate::day14::{day14work1};

    #[test]
    fn test_1() {
        match day14work1() {
            Ok(num) => println!("Day 14 Part 1 Sum: {num}"),
            Err(data) => panic!("Something went wrong: {}", data),
        }
    }
}