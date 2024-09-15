use std::path::PathBuf;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use framels_core::{Paths, PathsPacked};

fn generate_paths(n: u64) -> Paths {
    let mut paths = Vec::new();
    for i in 0..n {
        paths.push(PathBuf::from(format!("Beauty_{:0>8}.exr", i)));
    }
    assert_eq!(paths.len(), n as usize);
    Paths::from(paths)
}

fn criterion_benchmark(c: &mut Criterion) {
    #[allow(clippy::redundant_closure)]
    let mut group = c.benchmark_group("Parsing");
    for i in [1u64, 10u64, 100u64, 1000u64, 10000u64].iter() {
        let data_set = generate_paths(*i);
        group.bench_with_input(BenchmarkId::new("Mono", i), i, |b, _i| {
            b.iter(|| data_set.clone().pack(false))
        });
        group.bench_with_input(BenchmarkId::new("Multi", i), i, |b, _i| {
            b.iter(|| data_set.clone().pack(false))
        });
    }
    group.finish();
}
criterion_group!(name=benches;config = Criterion::default().significance_level(0.1).sample_size(100); targets=criterion_benchmark);
criterion_main!(benches);
