use advent_of_code_2022::day7::day7work1;
fn main() {
    match day7work1() {
        Ok(num) => println!("Sum: {num}"),
        Err(data) =>         panic!("Something went wrong: {}", data)
    }
}
