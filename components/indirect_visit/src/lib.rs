use std::ops::Index;

#[derive(Copy, Clone)]
pub struct Row {
    data_bytes: [u8; 64],
}

pub struct DirectRows {
    rows: Vec<Row>,
}

impl DirectRows {
    pub fn new_with_size(sz: usize) -> Self {
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

pub struct IndirectRows {
    indirect_offset: Vec<u32>,
    rows: Vec<u8>,
}

impl IndirectRows {
    pub fn new_with_size(sz: usize) -> Self {
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
            std::mem::transmute::<*const u8, &[u8; 64]>(
                self.rows
                    .as_ptr()
                    .offset(self.indirect_offset[index] as isize),
            )
        }
    }
}
