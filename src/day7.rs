use regex::Regex;
use std::fs;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Directory {
    name: String,
    subdirs: Vec<Directory>,
    size: usize,
}

impl Directory {
    fn new(name: String) -> Self {
        Self {
            name,
            subdirs: Vec::new(),
            size: 0,
        }
    }

    fn process<I>(&mut self, iter: &mut I) -> Result<usize, io::Error>
    where
        I: Iterator<Item = Result<String, io::Error>>,
    {
        let mut sizediff: usize = 0;
        let mut was_ls = false;
        while let Some(Ok(line)) = iter.next() {
            if line.starts_with('$') {
                if line.starts_with("$ cd") {
                    was_ls = false;
                    let dir = line.strip_prefix("$ cd ").unwrap().to_string();
                    if dir == ".." {
                        break;
                    }
                    if let Some(subdir) = self.subdirs.iter_mut().find(|i| -> bool {
                        return i.name == dir;
                    }) {
                        sizediff += subdir.process(iter)?;
                    } else {
                        self.subdirs.push(Directory::new(dir));
                        sizediff += self.subdirs.last_mut().unwrap().process(iter)?;
                    }
                } else if line.starts_with("$ ls") {
                    was_ls = true;
                }
            } else {
                assert!(was_ls);
                if line.starts_with("dir") {
                    let dir = line.strip_prefix("dir ").unwrap().to_string();
                    self.subdirs.push(Directory::new(dir));
                } else if let Some(capture) =
                    Regex::new(r"^(\d+) (.+)").unwrap().captures(line.as_str())
                {
                    let size: usize = capture.get(1).unwrap().as_str().parse().unwrap();
                    sizediff += size;
                }
            }
        }
        self.size += sizediff;
        return Ok(sizediff);
    }

    fn walk<F>(&mut self, f: &mut F)
    where
        F: FnMut(&mut Directory) -> (),
    {
        f(self);
        for sub in self.subdirs.iter_mut() {
            sub.walk(f);
        }
    }
}

pub fn day7work1() -> io::Result<usize> {
    let file = fs::File::open(&"data/day7.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut root = Directory {
        name: String::new(),
        size: 0,
        subdirs: vec![],
    };
    let _ = root.process(&mut lines)?;
    let mut sum = 0;
    root.walk(&mut |d| {
        if d.size <= 100000 {
            sum += d.size;
        }
    });
    return Ok(sum);
}

pub fn day7work2() -> io::Result<usize> {
    let file = fs::File::open(&"data/day7.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut root = Directory {
        name: String::new(),
        size: 0,
        subdirs: vec![],
    };
    let _ = root.process(&mut lines)?;
    let required = 30000000 + root.size - 70000000;
    let mut size = root.size;
    root.walk(&mut |d| {
        if d.size >= required && d.size < size {
            size = d.size;
        }
    });
    return Ok(size);
}

#[cfg(test)]
mod tests {
    use crate::day7::{day7work1, day7work2};

    #[test]
    fn test_1() {
        match day7work1() {
            Ok(num) => println!("Day 7 Part 1 Sum: {num}"),
            Err(data) => panic!("Something went wrong: {}", data),
        }
    }

    #[test]
    fn test_2() {
        match day7work2() {
            Ok(num) => println!("Day 7 Part 2 Result: {num}"),
            Err(data) => panic!("Something went wrong: {}", data),
        }
    }
}
