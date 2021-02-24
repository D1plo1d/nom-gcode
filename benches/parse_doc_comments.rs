use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nom_gcode::parse_gcode;

fn criterion_benchmark(c: &mut Criterion) {
    let comment = ";Filament used: 0.943758m";
    c.bench_function("parse_doc_comment", |b| b.iter(|| parse_gcode(black_box(comment))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
