use std::cmp::Ordering;

#[derive(PartialEq, Clone)]
pub enum Datum {
    Null,
    I64(i64),
    I32(i32),
    Bytes(Vec<u8>),
}

impl PartialOrd for Datum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Datum::Null, Datum::Null) => Some(Ordering::Equal),
            (Datum::Null, _) => Some(Ordering::Less),
            (Datum::I64(i1), Datum::I64(i2)) => i1.partial_cmp(i2),
            (Datum::I32(i1), Datum::I32(i2)) => i1.partial_cmp(i2),
            (Datum::Bytes(i1), Datum::Bytes(i2)) => i1.partial_cmp(i2),
            _ => unimplemented!(),
        }
    }
}

pub trait DatumVec {
    fn append(&mut self, datum: Datum);
    fn len(&self) -> usize;
    fn delete(&mut self, pos: usize);
    fn insert(&mut self, d: Datum);
    fn lower_bound(&self, d: &Datum) -> usize;
    fn upper_bound(&self, d: &Datum) -> usize;
}

#[derive(Clone, PartialOrd, PartialEq)]
pub struct NonChunkedVec {
    content: Vec<Datum>,
}

impl DatumVec for NonChunkedVec {
    fn append(&mut self, datum: Datum) {
        self.content.push(datum);
    }

    fn len(&self) -> usize {
        self.content.len()
    }

    fn delete(&mut self, pos: usize) {
        self.content.remove(pos);
    }

    fn insert(&mut self, d: Datum) {
        let i = self.content.partition_point(|v| v < &d);
        self.content.insert(i, d);
    }

    fn lower_bound(&self, d: &Datum) -> usize {
        self.content.partition_point(|v| v < d)
    }

    fn upper_bound(&self, d: &Datum) -> usize {
        self.content.partition_point(|v| v > d)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ChunkedVecSized<T: Sized> {
    data: Vec<T>,
}

impl<T: Sized> ChunkedVecSized<T> {
    pub fn new() -> Self {
        ChunkedVecSized {
            data: vec![]
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

pub enum ChunkedVec {
    I32Chunk(ChunkedVecSized<i32>),
    I64Chunk(ChunkedVecSized<i64>),
    BytesChunk(ChunkedVecSized<Vec<u8>>),
}

impl DatumVec for ChunkedVec {
    fn append(&mut self, datum: Datum) {
        match (datum, self) {
            (Datum::I32(i), ChunkedVec::I32Chunk(iv)) => {
                iv.data.push(i);
            }
            (Datum::I64(i), ChunkedVec::I64Chunk(iv)) => {
                iv.data.push(i);
            }
            (Datum::Bytes(i), ChunkedVec::BytesChunk(iv)) => {
                iv.data.push(i);
            }
            _ => unimplemented!(),
        }
    }

    fn len(&self) -> usize {
        match self {
            ChunkedVec::I32Chunk(v) => v.len(),
            ChunkedVec::I64Chunk(v) => v.len(),
            ChunkedVec::BytesChunk(v) => v.len(),
            _ => unimplemented!(),
        }
    }

    fn delete(&mut self, pos: usize) {
        todo!()
    }

    fn insert(&mut self, d: Datum) {
        todo!()
    }

    fn lower_bound(&self, d: &Datum) -> usize {
        todo!()
    }

    fn upper_bound(&self, d: &Datum) -> usize {
        todo!()
    }
}

pub trait SortKeys {
    fn append(&mut self, datum: NonChunkedVec);
    fn len(&self) -> usize;
    fn delete(&mut self, pos: usize);
    fn insert(&mut self, d: NonChunkedVec);
    fn lower_bound(&self, d: &NonChunkedVec) -> usize;
    fn upper_bound(&self, d: &NonChunkedVec) -> usize;
}

pub struct ChunkedSortKeys {
    data: Vec<ChunkedVec>,
}

pub struct NonChunkedSortKeys {
    data: Vec<NonChunkedVec>,
}

impl NonChunkedSortKeys {
    fn new() -> Self {
        NonChunkedSortKeys {
            data: vec![]
        }
    }
}

impl SortKeys for NonChunkedSortKeys {
    fn append(&mut self, datum: NonChunkedVec) {
        self.data.push(datum);
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn delete(&mut self, pos: usize) {
        self.data.remove(pos);
    }

    fn insert(&mut self, d: NonChunkedVec) {
        let i = self.data.partition_point(|v| v < &d);
        self.data.insert(i, d);
    }

    fn lower_bound(&self, d: &NonChunkedVec) -> usize {
        self.data.partition_point(|v| v < d)
    }

    fn upper_bound(&self, d: &NonChunkedVec) -> usize {
        self.data.partition_point(|v| v > d)
    }
}

impl ChunkedSortKeys {
    fn compare_idx(&self, idx: usize, other: &NonChunkedVec) -> Option<Ordering> {
        let mut current_index_idx = 0;

        match (
            &other.content[current_index_idx],
            &self.data[current_index_idx],
        ) {
            (Datum::I32(i), ChunkedVec::I32Chunk(iv)) => iv.data[idx].partial_cmp(i),
            (Datum::I64(i), ChunkedVec::I64Chunk(iv)) => iv.data[idx].partial_cmp(i),
            (Datum::Bytes(i), ChunkedVec::BytesChunk(iv)) => iv.data[idx].partial_cmp(i),
            _ => unimplemented!(),
        }
    }

    fn new() -> Self {
        ChunkedSortKeys { data: vec![] }
    }
}

impl SortKeys for ChunkedSortKeys {
    fn append(&mut self, datum: NonChunkedVec) {
        debug_assert!(self.data.len() == datum.content.len());
        let s = self.data.len();
        let mut idx = 0;
        for datum_item in datum.content {
            self.data[idx].append(datum_item);
            idx += 1;
        }
    }

    fn len(&self) -> usize {
        self.data[0].len()
    }

    fn delete(&mut self, pos: usize) {
        let s = self.data.len();
        for _ in 0..s {
            self.data[s].delete(pos);
        }
    }

    fn insert(&mut self, d: NonChunkedVec) {
        if self.data.len() != d.content.len() {
            panic!("datum count not matches");
        }
        let s = self.data.len();
        let mut idx = 0;
        for datum_item in d.content {
            self.data[idx].insert(datum_item);
            idx += 1;
        }
    }

    fn lower_bound(&self, d: &NonChunkedVec) -> usize {
        let mut upper = self.len();
        let mut lower = 0;
        while upper > lower {
            let current = lower + (upper - lower) / 2;
            match self.compare_idx(current, d).unwrap() {
                Ordering::Less => {
                    lower = current + 1;
                }
                Ordering::Equal => {
                    upper = current;
                }
                Ordering::Greater => {
                    upper = current;
                }
            }
        };
        lower
    }

    fn upper_bound(&self, d: &NonChunkedVec) -> usize {
        todo!()
    }
}

fn main() {
    use std::time::Instant;

    let mut data_vec = Vec::new();
    for i in 0..5000 {
        data_vec.push(NonChunkedVec { content: vec![Datum::I32(i)] });
    }

    let mut v1 = ChunkedSortKeys::new();
    {
        v1.data.push(ChunkedVec::I32Chunk(ChunkedVecSized::new()));
        let now = Instant::now();
        for i in 0..5000 {
            v1.append(data_vec[i].clone());
        }

        let elapse = now.elapsed();
        println!("d1 {:?}", elapse);
    }

    let mut v2 = NonChunkedSortKeys::new();
    {
        let now = Instant::now();
        for i in 0..5000 {
            v2.append(data_vec[i].clone());
        }

        let elapse = now.elapsed();
        println!("d2 {:?}", elapse);
    }

    {
        let now = Instant::now();
        for i in 0..5000 {
            v1.lower_bound(&data_vec[i]);
        }

        let elapse = now.elapsed();
        println!("d1-2 {:?}", elapse);
    }

    {
        let now = Instant::now();
        for i in 0..5000 {
            v2.lower_bound(&data_vec[i]);
        }

        let elapse = now.elapsed();
        println!("d2-2 {:?}", elapse);
    }
}
