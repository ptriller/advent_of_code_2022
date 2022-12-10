use advent_of_code_2022::day8::day8work1;
fn main() {
    match day8work1() {
        Ok(num) => println!("Count: {num}"),
        Err(data) =>         panic!("Something went wrong: {}", data)
    }
}
