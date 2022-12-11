use advent_of_code_2022::day9::day9work2;
fn main() {
    match day9work2() {
        Ok(num) => println!("Positions: {num}"),
        Err(data) =>         panic!("Something went wrong: {}", data)
    }
}
