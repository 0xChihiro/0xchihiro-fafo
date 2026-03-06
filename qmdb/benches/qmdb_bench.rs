use criterion::{Criterion, criterion_group, criterion_main};
use qmdb::utils::bytescache::split_cache_pos;

fn cache_split(c: &mut Criterion) {
    c.bench_function("cache split", |b| {
        b.iter(|| split_cache_pos(32490723479123))
    });
}

criterion_group!(benches, cache_split);
criterion_main!(benches);
