use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};

use enum_compare::*;

fn non_chunked_push_back(cnt: u64) -> NonChunkedSortKeys {
    let mut data_vec = Vec::new();
    for i in 0..cnt {
        data_vec.push(NonChunkedVec {
            content: vec![Datum::I32(i as i32), Datum::I64(i as i64)],
        });
    }

    let mut v1 = NonChunkedSortKeys::new();
    v1.reserve(cnt as usize);
    for i in 0..cnt {
        v1.append(data_vec[i as usize].clone());
    }
    v1
}

fn chunked_push_back(cnt: u64) -> ChunkedSortKeys {
    let mut data_vec = Vec::new();
    for i in 0..cnt {
        data_vec.push(NonChunkedVec {
            content: vec![Datum::I32(i as i32), Datum::I64(i as i64)],
        });
    }

    let mut v1 = ChunkedSortKeys::new();
    v1.data.push(ChunkedVec::I32Chunk(ChunkedVecSized::new()));
    v1.data.push(ChunkedVec::I64Chunk(ChunkedVecSized::new()));
    v1.reserve(cnt as usize);
    for i in 0..cnt {
        v1.append(data_vec[i as usize].clone());
    }
    v1
}

fn searching_data<ChunkedT: SortKeys>(data_vec: &Vec<NonChunkedVec>, searching_chunk: &ChunkedT) {
    for dv in data_vec {
        let x = searching_chunk.lower_bound(dv);
        black_box(x);
    }
}

fn bench_push_back(c: &mut Criterion) {
    let mut group = c.benchmark_group("SortKeys Inserts");
    for i in [500u64, 1500u64].iter() {
        group.bench_with_input(BenchmarkId::new("Unchunked", i), i, |b, i| {
            b.iter(|| non_chunked_push_back(*i))
        });
        group.bench_with_input(BenchmarkId::new("Chunked", i), i, |b, i| {
            b.iter(|| chunked_push_back(*i))
        });
    }
    group.finish();
}

fn bench_searching_idx(c: &mut Criterion) {
    let mut group = c.benchmark_group("SortKeys Search");

    for cnt in [500u64, 1500u64].iter() {
        let mut data_vec = Vec::new();
        for i in 0..*cnt {
            data_vec.push(NonChunkedVec {
                content: vec![Datum::I32(i as i32), Datum::I64(i as i64)],
            });
        }
        let non_chunked = non_chunked_push_back(*cnt);
        let chunked = chunked_push_back(*cnt);
        group.bench_function(BenchmarkId::new("Unchunked", cnt), |b| {
            b.iter(|| searching_data(&data_vec, &non_chunked))
        });

        group.bench_function(BenchmarkId::new("Chunked", cnt), |b| {
            b.iter(|| searching_data(&data_vec, &chunked))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_push_back, bench_searching_idx);
criterion_main!(benches);
