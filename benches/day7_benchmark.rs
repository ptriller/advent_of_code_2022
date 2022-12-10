use criterion::{black_box, criterion_group, criterion_main, Criterion};
use advent_of_code_2022::day7::{day7work1, day7work2};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day7a", |b| b.iter(|| day7work1()));
    c.bench_function("day7b", |b| b.iter(|| day7work2()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);