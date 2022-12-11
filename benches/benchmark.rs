use criterion::{criterion_group, criterion_main, Criterion};
use advent_of_code_2022::day1::{day1work1, day1work2};
use advent_of_code_2022::day10::{day10work1, day10work2};
use advent_of_code_2022::day11::{day11work1, day11work2};
use advent_of_code_2022::day2::{day2work1, day2work2};
use advent_of_code_2022::day3::{day3work1, day3work2};
use advent_of_code_2022::day4::{day4work1, day4work2};
use advent_of_code_2022::day5::{day5work1, day5work2};
use advent_of_code_2022::day6::{day6work1, day6work2};
use advent_of_code_2022::day7::{day7work1, day7work2};
use advent_of_code_2022::day8::{day8work1, day8work2};
use advent_of_code_2022::day9::{day9work1, day9work2};

pub fn day1_benchmark(c: &mut Criterion) {
    c.bench_function("day1a", |b| b.iter(|| day1work1()));
    c.bench_function("day1b", |b| b.iter(|| day1work2()));
}

pub fn day2_benchmark(c: &mut Criterion) {
    c.bench_function("day2a", |b| b.iter(|| day2work1()));
    c.bench_function("day2b", |b| b.iter(|| day2work2()));
}

pub fn day3_benchmark(c: &mut Criterion) {
    c.bench_function("day3a", |b| b.iter(|| day3work1()));
    c.bench_function("day3b", |b| b.iter(|| day3work2()));
}

pub fn day4_benchmark(c: &mut Criterion) {
    c.bench_function("day4a", |b| b.iter(|| day4work1()));
    c.bench_function("day4b", |b| b.iter(|| day4work2()));
}

pub fn day5_benchmark(c: &mut Criterion) {
    c.bench_function("day5a", |b| b.iter(|| day5work1()));
    c.bench_function("day5b", |b| b.iter(|| day5work2()));
}

pub fn day6_benchmark(c: &mut Criterion) {
    c.bench_function("day6a", |b| b.iter(|| day6work1()));
    c.bench_function("day6b", |b| b.iter(|| day6work2()));
}

pub fn day7_benchmark(c: &mut Criterion) {
    c.bench_function("day7a", |b| b.iter(|| day7work1()));
    c.bench_function("day7b", |b| b.iter(|| day7work2()));
}

pub fn day8_benchmark(c: &mut Criterion) {
    c.bench_function("day8a", |b| b.iter(|| day8work1()));
    c.bench_function("day8b", |b| b.iter(|| day8work2()));
}


pub fn day9_benchmark(c: &mut Criterion) {
    c.bench_function("day9a", |b| b.iter(|| day9work1()));
    c.bench_function("day9b", |b| b.iter(|| day9work2()));
}

pub fn day10_benchmark(c: &mut Criterion) {
    c.bench_function("day10a", |b| b.iter(|| day10work1()));
    c.bench_function("day10b", |b| b.iter(|| day10work2()));
}

pub fn day11_benchmark(c: &mut Criterion) {
    c.bench_function("day11a", |b| b.iter(|| day11work1()));
    c.bench_function("day11b", |b| b.iter(|| day11work2()));
}


criterion_group!(benches,
    day1_benchmark,
    day2_benchmark,
    day3_benchmark,
    day4_benchmark,
    day5_benchmark,
    day6_benchmark,
    day7_benchmark,
    day8_benchmark,
    day9_benchmark,
    day10_benchmark);
criterion_main!(benches);