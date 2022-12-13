use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Position(usize, usize);

#[derive(Debug)]
struct Terrain {
    start: Position,
    end: Position,
    map: Vec<Vec<isize>>,
}

impl Terrain {
    fn new<I>(lines: I) -> Self
        where
            I: Iterator<Item=Result<String, io::Error>>, {
        let mut terrain = Terrain {
            start: Position(0, 0),
            end: Position(0, 0),
            map: vec!(),
        };
        let mut lnum = 0;
        for res in lines {
            let line = res.unwrap();
            let mut cnum = 0;
            let mut row = vec!();
            for c in line.chars() {
                row.push(match c {
                    'S' => {
                        terrain.start = Position(lnum, cnum);
                        0
                    }
                    'E' => {
                        terrain.end = Position(lnum, cnum);
                        'z' as isize - 'a' as isize
                    }
                    c => c as isize - 'a' as isize
                });
                cnum += 1;
            }
            terrain.map.push(row);
            lnum += 1;
        }
        terrain
    }

    fn find_distance(&mut self, start: Vec<Position>) -> io::Result<usize>
    {
        let mut track = vec![vec![usize::MAX as usize; self.map[0].len()]; self.map.len()];
        let mut edge: Box<HashSet<Position>> = Box::new(HashSet::from_iter(start));
        let mut step = 0;
        for x in edge.iter() {
            track[x.0][x.1] = step;
        }
        'outer:
        loop {
            assert!(edge.len() > 0);
            let mut next_step = Box::new(HashSet::new());
            for pos in edge.iter() {
                if pos.0 == self.end.0 && pos.1 == self.end.1 {
                    break 'outer;
                }
                if pos.0 > 0 && track[pos.0 - 1][pos.1] == usize::MAX
                    && self.map[pos.0 - 1][pos.1] - self.map[pos.0][pos.1] <= 1 {
                    track[pos.0 - 1][pos.1] = step;
                    next_step.insert(Position(pos.0 - 1, pos.1));
                }
                if pos.0 < track.len() - 1 && track[pos.0 + 1][pos.1] == usize::MAX
                    && self.map[pos.0 + 1][pos.1] - self.map[pos.0][pos.1] <= 1 {
                    track[pos.0 + 1][pos.1] = step;
                    next_step.insert(Position(pos.0 + 1, pos.1));
                }
                if pos.1 > 0 && track[pos.0][pos.1 - 1] == usize::MAX
                    && self.map[pos.0][pos.1 - 1] - self.map[pos.0][pos.1] <= 1 {
                    track[pos.0][pos.1 - 1] = step;
                    next_step.insert(Position(pos.0, pos.1 - 1));
                }
                if pos.1 < track[pos.0].len() - 1 && track[pos.0][pos.1 + 1] == usize::MAX
                    && self.map[pos.0][pos.1 + 1] - self.map[pos.0][pos.1] <= 1 {
                    track[pos.0][pos.1 + 1] = step;
                    next_step.insert(Position(pos.0, pos.1 + 1));
                }
            }
            step += 1;
            edge = next_step;
        }
        return Ok(step);
    }

    fn find_elevation(&self, elevation: isize) -> Vec<Position> {
        let mut result = vec!();
        for x in 0..self.map.len() {
            for y in 0..self.map[x].len() {
                if self.map[x][y] == elevation {
                    result.push(Position(x, y));
                }
            }
        }
        result
    }

}

pub fn day12work1() -> io::Result<usize> {
    let file = File::open(&"data/day12.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut terrain = Terrain::new(&mut lines);
    return terrain.find_distance(vec![terrain.start]);
}

pub fn day12work2() -> io::Result<usize> {
    let file = File::open(&"data/day12.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut terrain = Terrain::new(&mut lines);
    let valleys = terrain.find_elevation(0);
    return terrain.find_distance(valleys);
}


#[cfg(test)]
mod tests {
    use crate::day12::{day12work1, day12work2};

    #[test]
    fn test_1() {
        match day12work1() {
            Ok(num) => println!("Day 12 Part 1 Steps: {num}"),
            Err(data) => panic!("Something went wrong: {}", data)
        }
    }

    #[test]
    fn test_2() {
        match day12work2() {
            Ok(num) => println!("Day 12 Part 2 Steps: {num}"),
            Err(data) => panic!("Something went wrong: {}", data)
        }
    }
}