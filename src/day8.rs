use std::fs;
use std::io::{self, BufRead};

struct Forrest {
    trees: Vec<Vec<u32>>,
}

impl Forrest {
    fn from_iter<I>(iter: &mut I) -> Self
        where
            I: Iterator<Item=Result<String, io::Error>>, {
        let mut forrest = Forrest { trees: vec!() };
        for result in iter {
            forrest.trees.push(
                result.unwrap().chars().map(|c| c as u32 - '0' as u32).collect()
            );
        }
        forrest
    }

    fn max_scenic(&self) -> usize {
        let mut max = 0;
        for y in 0..self.trees.len() {
            for x in 0..self.trees[y].len() {
                let score = self.scenic_score(x,y);
                if score > max {
                    max = score;
                }
            }
        }
        return max;
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let mut left = 0;
        let height = self.trees[y][x];
        for dx in (0..x).rev() {
            left += 1;
            if self.trees[y][dx] >= height {
                break;
            }
        }
        let mut right = 0;
        for dx in x+1..self.trees[y].len() {
            right += 1;
            if self.trees[y][dx] >= height {
                break;
            }
        }
        let mut top = 0;
        for dy in (0..y).rev() {
            top += 1;
            if self.trees[dy][x] >= height {
                break;
            }
        }
        let mut bottom = 0;
        for dy in y+1..self.trees.len() {
            bottom += 1;
            if self.trees[dy][x] >= height {
                break;
            }
        }
        return left*right*top*bottom;
    }

    fn count_visibles(&self) -> usize {
        let mut count = 0;
        for i in 0..self.trees.len() {
            let mut rowcount = 0;
            for j in 0..self.trees[i].len() {
                if self.is_visible(i, j) {
                    rowcount += 1;
                }
            }
            count += rowcount;
        }
        return count;
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        let height = self.trees[y][x];
        return self.check_x_range(height, y, 0, x)
            || self.check_x_range(height, y, x + 1, self.trees[y].len())
            || self.check_y_range(height, x, 0, y)
            || self.check_y_range(height, x, y + 1, self.trees.len());
    }

    fn check_x_range(&self, height: u32, y: usize, from: usize, to: usize) -> bool {
        for x in from..to {
            if self.trees[y][x] >= height {
                return false;
            }
        }
        return true;
    }

    fn check_y_range(&self, height: u32, x: usize, from: usize, to: usize) -> bool {
        for y in from..to {
            if self.trees[y][x] >= height {
                return false;
            }
        }
        return true;
    }
}

pub fn day8work1() -> io::Result<usize> {
    let file = fs::File::open(&"data/day8.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let forrest = Forrest::from_iter(&mut lines);
    return Ok(forrest.count_visibles());
}

pub fn day8work2() -> io::Result<usize> {
    let file = fs::File::open(&"data/day8.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let forrest = Forrest::from_iter(&mut lines);
    return Ok(forrest.max_scenic());
}