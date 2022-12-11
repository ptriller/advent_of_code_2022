use criterion::{criterion_group, criterion_main, Criterion};
use advent_of_code_2022::day6::{day6work1, day6work2};
use advent_of_code_2022::day7::{day7work1, day7work2};
use advent_of_code_2022::day8::{day8work1, day8work2};
use advent_of_code_2022::day9::{day9work1, day9work2};

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


criterion_group!(benches,  day6_benchmark,day7_benchmark, day8_benchmark, day9_benchmark);
criterion_main!(benches);