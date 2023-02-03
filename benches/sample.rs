use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fil_ls::{parse_dir, run};

fn parse_and_run() {
    let paths: Vec<String> = parse_dir("./samples".to_string());
    let results: Vec<String> = run(paths);
    println!("{}", results.join("\n"))
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fls 20", |b| b.iter(|| parse_and_run()));
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
