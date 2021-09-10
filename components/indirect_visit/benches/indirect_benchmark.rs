use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use std::ops::Index;

use indirect_visit::*;

fn traverse_in_range<Idx: Index<usize, Output = [u8; 64]>>(rows: &Idx, size: usize) {
    for idx in 0..size {
        black_box(rows[idx]);
    }
}

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("VisitingArray");
    for i in [200, 500, 1000, 1500].iter() {
        let g1 = DirectRows::new_with_size(*i as usize);
        let g2 = IndirectRows::new_with_size(*i as usize);
        group.bench_with_input(BenchmarkId::new("Direct", i), i, |b, i| {
            b.iter(|| traverse_in_range(&g1, *i as usize))
        });
        group.bench_with_input(BenchmarkId::new("Indirect", i), i, |b, i| {
            b.iter(|| traverse_in_range(&g2, *i as usize))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
