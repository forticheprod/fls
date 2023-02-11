use criterion::{criterion_group, criterion_main, Criterion};
use framels::{basic, parse_dir};

fn parse_and_run() {
    let paths: Vec<String> = parse_dir("./samples/big".to_string());
    let _results: Vec<String> = basic(paths);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("big", |b| b.iter(|| parse_and_run()));
}
criterion_group!(name=benches;config = Criterion::default().significance_level(0.1).sample_size(100); targets=criterion_benchmark);
criterion_main!(benches);
