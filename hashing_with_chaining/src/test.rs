use criterion::{criterion_group, criterion_main, Criterion};
mod main;

pub fn criterion_random_gen(c: &mut Criterion) {
    c.bench_function("Random Generator", |b| b.iter(|| main::random_generator(0, 10_000)));
}

criterion_group!(benches, criterion_random_gen);
criterion_main!(benches);