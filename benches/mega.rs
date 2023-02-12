use criterion::{criterion_group, criterion_main, Criterion};
use framels::basic;
use serde::Deserialize;

#[derive(Deserialize)]
struct Paths {
    paths_list: Vec<String>,
}

fn get_data_set() -> Vec<String> {
    let text = std::fs::read_to_string("/home/philippellerena/Downloads/e101.json").unwrap();
    let dataset = serde_json::from_str::<Paths>(&text).unwrap();
    dataset.paths_list
}

fn parse_and_run() {
    let _results: Vec<String> = basic(get_data_set());
}

fn criterion_benchmark(c: &mut Criterion) {
    #[allow(clippy::redundant_closure)]
    c.bench_function("mega", |b| b.iter(|| parse_and_run()));
}
criterion_group!(name=benches;config = Criterion::default().significance_level(0.1).sample_size(10); targets=criterion_benchmark);
criterion_main!(benches);
