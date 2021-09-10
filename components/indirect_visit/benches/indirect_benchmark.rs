use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};

use std::ops::Index;

#[derive(Copy, Clone)]
pub struct Row {
    data_bytes: [u8; 64],
}

struct DirectRows {
    rows: Vec<Row>,
}

impl DirectRows {
    fn new_with_size(sz: usize) -> Self {
        let mut rows = Vec::with_capacity(sz);
        rows.resize(
            sz,
            Row {
                data_bytes: [0u8; 64],
            },
        );
        DirectRows { rows }
    }
}

struct IndirectRows {
    indirect_offset: Vec<u32>,
    rows: Vec<u8>,
}

impl IndirectRows {
    fn new_with_size(sz: usize) -> Self {
        let mut rows = Vec::with_capacity(sz * 64);
        let mut indirect_offset: Vec<u32> = Vec::with_capacity(sz);
        for i in 0..sz {
            indirect_offset.push((i * 64) as u32);
        }
        rows.resize(sz * 64, 0);

        IndirectRows {
            indirect_offset,
            rows,
        }
    }
}

impl Index<usize> for DirectRows {
    type Output = [u8; 64];

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index].data_bytes
    }
}

impl Index<usize> for IndirectRows {
    type Output = [u8; 64];

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            std::mem::transmute::<usize, &[u8; 64]>(
                self.rows
                    .as_ptr()
                    .align_offset(self.indirect_offset[index] as usize),
            )
        }
    }
}

fn traverse_in_range<Idx: Index<usize, Output=[u8; 64]>>(rows: &Idx, size: usize) {
    for idx in 0..size {
        black_box(rows[idx]);
    }
}

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci");
    for i in [200, 500, 1000, 1500].iter() {
        let g1 = DirectRows::new_with_size(*i as usize);
        let g2 = IndirectRows::new_with_size(*i as usize);
        group.bench_with_input(BenchmarkId::new("Recursive", i), i, |b, i| {
            b.iter(|| traverse_in_range(&g1, *i as usize))
        });
        group.bench_with_input(BenchmarkId::new("Iterative", i), i, |b, i| {
            b.iter(||  traverse_in_range(&g2, *i as usize))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
