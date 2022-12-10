use advent_of_code_2022::day6::day6work2;
fn main() {
    match day6work2() {
        Ok(num) => println!("Processed: {num}"),
        Err(data) =>         panic!("Something went wrong: {}", data)
    }
}
