use advent_of_code_2022::day6::day6work1;
fn main() {
    match day6work1() {
        Ok(num) => println!("Processed: {num}"),
        Err(data) =>         panic!("Something went wrong: {}", data)
    }
}
