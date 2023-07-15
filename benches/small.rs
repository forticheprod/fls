use criterion::{criterion_group, criterion_main, Criterion};
use framels::{
    basic_listing, parse_dir,
    paths::{Paths, PathsPacked},
};

fn parse_and_run() {
    let source = "./samples/big".to_string();
    let paths: Paths = parse_dir(&source);
    let _results: PathsPacked = basic_listing(paths);
}

fn criterion_benchmark(c: &mut Criterion) {
    #[allow(clippy::redundant_closure)]
    c.bench_function("small", |b| b.iter(|| parse_and_run()));
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
