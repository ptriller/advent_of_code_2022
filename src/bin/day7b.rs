use advent_of_code_2022::day7::day7work2;
fn main() {
    match day7work2() {
        Ok(num) => println!("Result: {num}"),
        Err(data) =>         panic!("Something went wrong: {}", data)
    }
}
