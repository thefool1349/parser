use criterion::{Criterion, criterion_group, criterion_main};
use parser::parser::parser;
use std::hint::black_box;

fn benchmark_parser(c: &mut Criterion) {
    c.bench_function("CSV parser", |b| {
        b.iter(|| {
            // Put your parsing logic here for now.
            black_box(parser().unwrap());
        });
    });
}

criterion_group!(benches, benchmark_parser);
criterion_main!(benches);
