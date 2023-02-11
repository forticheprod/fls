use criterion::{criterion_group, criterion_main, Criterion};
use framels::{basic, parse_dir};

fn parse_and_run() {
    let paths: Vec<String> = parse_dir("./samples/small/".to_string());
    let _results: Vec<String> = basic(paths);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("small", |b| b.iter(|| parse_and_run()));
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
