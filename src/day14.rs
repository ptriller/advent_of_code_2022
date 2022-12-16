use std::fs::File;
use std::io;
use std::io::{BufRead, Cursor, Read, Seek, SeekFrom};
use image::{EncodableLayout, Rgb, RgbImage};
use minimp4::Mp4Muxer;
use openh264::encoder::{Encoder, EncoderConfig};
use openh264::formats::YUVBuffer;


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
            map: vec![vec!['.'; 1100]; 1100],
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
                result.map[pos.0][pos.1] = '#';
            }
        }
        return result;
    }

    fn add_sand(&mut self) -> bool {
        return self.add_sand_ex(&mut |_, _| ());
    }

    fn add_sand_ex<F: FnMut(&mut Cave, &(usize, usize))>(&mut self, tick: &mut F) -> bool {
        let mut sand = (500, 0);
        if self.map[sand.0][sand.1] != '.' {
            tick(self, &sand);
            return false;
        }
        let mut i = 0;
        while self.bottom >= sand.1 && self.left <= sand.0 && self.right >= sand.0 {
            if self.map[sand.0][sand.1 + 1] == '.' {
                sand = (sand.0, sand.1 + 1);
            } else if self.map[sand.0 - 1][sand.1 + 1] == '.' {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if self.map[sand.0 + 1][sand.1 + 1] == '.' {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                self.map[sand.0][sand.1] = 'o';
                tick(self, &sand);
                return true;
            }
            if i % 10 == 0 {
                tick(self, &sand);
            }
            i += 1;
        }
        return false;
    }

    fn add_floor(&mut self) {
        for i in 0..self.map[self.bottom + 2].len() {
            self.map[i][self.bottom + 2] = '#';
        }
        self.bottom += 2;
        self.right = 500 + self.bottom + 2;
        self.left = 500 - self.bottom - 2;
    }

    fn draw(&self) {
        for i in 0..self.bottom + 2 {
            for j in self.left - 1..self.right + 2 {
                print!("{}", self.map[j][i])
            }
            println!()
        }
    }

    fn parse_pos(pair: &str) -> (usize, usize) {
        let mut num = pair.split(",");
        return (num.next().unwrap().parse().unwrap(),
                num.next().unwrap().parse().unwrap());
    }

    fn generate_frame(&self, sand: &(usize, usize)) -> RgbImage {
        let mut image = RgbImage::new(
            2 * (self.right - self.left + 3) as u32, 2 * (self.bottom + 2) as u32);
        for i in 0..self.bottom + 2 {
            for j in self.left - 1..self.right + 2 {
                let mut px = match self.map[j][i] {
                    _ if sand.0 == j && sand.1 == i => {
                        Rgb([255u8, 0u8, 0u8])
                    }
                    '.' => Rgb([0u8, 0u8, 0u8]),
                    '#' => Rgb([140u8, 140u8, 140u8]),
                    'o' => Rgb([255u8, 204u8, 102u8]),
                    _ => panic!("xxxx")
                };
                image.put_pixel(2 * (j + 1 - self.left) as u32, 2 * i as u32, px);
                image.put_pixel(2 * (j + 1 - self.left) as u32 + 1, 2 * i as u32, px);
                image.put_pixel(2 * (j + 1 - self.left) as u32, 2 * i as u32 + 1, px);
                image.put_pixel(2 * (j + 1 - self.left) as u32 + 1, 2 * i as u32 + 1, px);
            }
        }
        return image;
    }
}

pub fn day14work1() -> io::Result<usize> {
    let file = File::open(&"data/day14.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut cave = Cave::new(&mut lines);
    let mut count = 0;
    while cave.add_sand() {
        count += 1;
    }
    cave.draw();
    Ok(count)
}

pub fn day14work1_vid() -> io::Result<usize> {
    let config = EncoderConfig::new(128, 336);
    let mut encoder = Encoder::with_config(config).unwrap();

    let file = File::open(&"data/day14.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut cave = Cave::new(&mut lines);
    let mut count = 0;
    let mut buf = Vec::new();
    while cave.add_sand_ex(&mut |cref, sand| {
        let data = cref.generate_frame(sand);
        let mut yuv = YUVBuffer::with_rgb(data.width() as usize, data.height() as usize, data.as_bytes());
        let bitstream = encoder.encode(&yuv).unwrap();
        bitstream.write_vec(&mut buf);
    }) {
        count += 1;
    }
    let mut video_buffer = Cursor::new(Vec::new());
    let mut mp4muxer = Mp4Muxer::new(&mut video_buffer);
    mp4muxer.init_video(2 * (cave.right - cave.left + 3) as i32,
                        2 * (cave.bottom + 2) as i32,
                        false,
                        "Falling sand.");
    mp4muxer.write_video(&buf);
    mp4muxer.close();
    // Some shenanigans to get the raw bytes for the video.
    video_buffer.seek(SeekFrom::Start(0)).unwrap();
    let mut video_bytes = Vec::new();
    video_buffer.read_to_end(&mut video_bytes).unwrap();

    std::fs::write("/tmp/out/sand.mp4", &video_bytes).unwrap();
    cave.draw();
    Ok(count)
}

pub fn day14work2() -> io::Result<usize> {
    let file = File::open(&"data/day14.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let mut cave = Cave::new(&mut lines);
    let mut count = 0;
    cave.add_floor();
    while cave.add_sand() {
        count += 1;
    }
    cave.draw();
    Ok(count)
}


#[cfg(test)]
mod test {
    use crate::day14::{day14work1, day14work1_vid, day14work2};

    #[test]
    fn test_1() {
        match day14work1() {
            Ok(num) => println!("Day 14 Part 1 Sum: {num}"),
            Err(data) => panic!("Something went wrong: {}", data),
        }
    }

    #[test]
    fn test_2() {
        match day14work2() {
            Ok(num) => println!("Day 14 Part 2 Sum: {num}"),
            Err(data) => panic!("Something went wrong: {}", data),
        }
    }

    //    #[test]
    fn test_3() {
        match day14work1_vid() {
            Ok(num) => println!("Day 14 Part 1 Sum: {num}"),
            Err(data) => panic!("Something went wrong: {}", data),
        }
    }
}