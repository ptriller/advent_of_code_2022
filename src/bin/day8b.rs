use advent_of_code_2022::day8::day8work2;
fn main() {
    match day8work2() {
        Ok(num) => println!("Max Scenic: {num}"),
        Err(data) =>         panic!("Something went wrong: {}", data)
    }
}
