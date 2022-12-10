use criterion::{criterion_group, criterion_main, Criterion};
use advent_of_code_2022::day6::{day6work1, day6work2};
use advent_of_code_2022::day7::{day7work1, day7work2};

pub fn day6_benchmark(c: &mut Criterion) {
    c.bench_function("day6a", |b| b.iter(|| day6work1()));
    c.bench_function("day6b", |b| b.iter(|| day6work2()));
}

pub fn day7_benchmark(c: &mut Criterion) {
    c.bench_function("day7a", |b| b.iter(|| day7work1()));
    c.bench_function("day7b", |b| b.iter(|| day7work2()));
}

criterion_group!(benches,  day6_benchmark,day7_benchmark);
criterion_main!(benches);