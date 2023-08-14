use criterion::{criterion_group, criterion_main, Criterion};
use framels::{
    basic_listing, parse_dir,
    paths::{Paths, PathsPacked},
};
use serde::Deserialize;

#[derive(Deserialize)]
struct PathsJson {
    paths_list: Vec<String>,
}
impl PathsJson {
    pub fn to_paths(&self) -> Paths {
        Paths::new(self.paths_list.clone())
    }
}

fn get_data_set() -> Paths {
    let text = std::fs::read_to_string("./samples/mega/response_1689510067951.json").unwrap();
    let dataset = serde_json::from_str::<PathsJson>(&text).unwrap();
    dataset.to_paths()
}

fn mega_parse_and_run() {
    let _results: PathsPacked = basic_listing(get_data_set());
}
fn parse_and_run() {
    let source = "./samples/big".to_string();
    let paths: Paths = parse_dir(&source);
    let _results: PathsPacked = basic_listing(paths);
}

fn small_parse_and_run() {
    let source = "./samples/big".to_string();
    let paths: Paths = parse_dir(&source);
    let _results: PathsPacked = basic_listing(paths);
}

fn criterion_benchmark(c: &mut Criterion) {
    #[allow(clippy::redundant_closure)]
    c.bench_function("json", |b| b.iter(|| mega_parse_and_run()));
    c.bench_function("big", |b| b.iter(|| parse_and_run()));
    c.bench_function("small", |b| b.iter(|| small_parse_and_run()));
}
criterion_group!(name=benches;config = Criterion::default().significance_level(0.1).sample_size(100); targets=criterion_benchmark);
criterion_main!(benches);
