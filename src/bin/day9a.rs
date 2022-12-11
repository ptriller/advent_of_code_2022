use advent_of_code_2022::day9::day9work1;
fn main() {
    match day9work1() {
        Ok(num) => println!("Positopms: {num}"),
        Err(data) =>         panic!("Something went wrong: {}", data)
    }
}
