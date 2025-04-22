use std::path::PathBuf;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use framels::{
    basic_listing, extended_listing, parse_dir,
    paths::{Paths, PathsPacked},
    FormatTemplate,
};

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
    let _results: PathsPacked = basic_listing(paths, false, FormatTemplate::default().format);
}

fn small_parse_and_run() {
    let source = "./samples/big".to_string();
    let paths: Paths = parse_dir(&source);
    let _results: PathsPacked = basic_listing(paths, false, FormatTemplate::default().format);
}

fn exr_reading() {
    let source = "./samples/big/".to_string();
    let paths: Paths = parse_dir(&source);
    let _results: PathsPacked = extended_listing(source, paths, false, FormatTemplate::default().format);
}

fn criterion_benchmark(c: &mut Criterion) {
    #[allow(clippy::redundant_closure)]
    let mut group = c.benchmark_group("Parsing");
    for i in [1u64, 10u64, 100u64, 1000u64, 10000u64].iter() {
        let data_set = generate_paths(*i);
        group.bench_with_input(BenchmarkId::new("Mono", i), i, |b, _i| {
            b.iter(|| basic_listing(data_set.clone(), false,  FormatTemplate::default().format))
        });
        group.bench_with_input(BenchmarkId::new("Multi", i), i, |b, _i| {
            b.iter(|| basic_listing(data_set.clone(), true, FormatTemplate::default().format))
        });
    }
    group.finish();
    c.bench_function("big", |b| b.iter(|| parse_and_run()));
    c.bench_function("small", |b| b.iter(|| small_parse_and_run()));
    c.bench_function("exr_reading", |b| b.iter(|| exr_reading()));
}
criterion_group!(name=benches;config = Criterion::default().significance_level(0.1).sample_size(100); targets=criterion_benchmark);
criterion_main!(benches);
