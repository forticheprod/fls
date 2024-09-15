use std::path::PathBuf;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use framels::{extended_listing, parse_dir, PathsPackedMeta};
use framels_core::{Paths, PathsPacked};
use predicates::path;

fn generate_paths(n: u64) -> Paths {
    let mut paths = Vec::new();
    for i in 0..n {
        paths.push(PathBuf::from(format!("Beauty_{:0>8}.exr", i)));
    }
    assert_eq!(paths.len(), n as usize);
    Paths::from(paths)
}

fn parse_and_run() {
    let source = "./samples/big".to_string();
    let paths: Paths = parse_dir(&source);
    let _results: PathsPacked = paths.pack(false);
}

fn small_parse_and_run() {
    let source = "./samples/big".to_string();
    let paths: Paths = parse_dir(&source);
    let _results: PathsPacked = paths.pack(false);
}

fn exr_reading() {
    let source = "./samples/big/".to_string();
    let paths: Paths = parse_dir(&source);
    let _results: PathsPackedMeta = extended_listing(source, paths, false);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("big", |b| b.iter(|| parse_and_run()));
    c.bench_function("small", |b| b.iter(|| small_parse_and_run()));
    c.bench_function("exr_reading", |b| b.iter(|| exr_reading()));
}
criterion_group!(name=benches;config = Criterion::default().significance_level(0.1).sample_size(100); targets=criterion_benchmark);
criterion_main!(benches);
