use std::time;

use bit_field::BitField;

// // Epoch is set to the twitter snowflake epoch of Nov 04 2010 01:42:54 UTC in milliseconds
// // You may customize this to set a different epoch for your application.
// const EPOCH: u64 = 1288834974657;

// TODO(mwish): Think of can we use atomic and cas.
pub struct Node {
    // immutable since start
    epoch: time::Instant,

    node_id: i32,
    seq_id: i32,
    time: Option<i64>,
}

impl Node {
    pub fn new(node_id: i32) -> Result<Self, time::SystemTimeError> {
        // let t1 = time::UNIX_EPOCH + time::Duration::from_millis(EPOCH);
        // println!("{:?} {:?}", t1, time::UNIX_EPOCH);
        // // Duration
        // let d = time::SystemTime::now()
        //     .duration_since(t1)?;
        // println!("{:?}", d);
        // // Note: If time is earlier than d, it doesn't work.
        // println!("{:?}", time::Instant::now());
        // let epoch = time::Instant::now() - d;
        let epoch = time::Instant::now();

        Ok(Node {
            epoch,
            node_id,
            seq_id: 0,
            time: None,
        })
    }

    pub fn generate(&mut self) -> Id {
        let timestamp = (self.epoch.elapsed().as_nanos() / 1000000) as i64;
        let mut changed = false;
        match self.time {
            None => {
                changed = true;
            },
            Some(ref t) => {
                if *t != timestamp {
                    changed = true;
                }
            }
        }
        let seq_id: i32;
        if changed {
            self.time = Some(timestamp);
            seq_id = 0;
            self.seq_id = 1;
        } else {
            seq_id = self.seq_id;
            self.seq_id += 1;
        }

        Id::new(self.node_id, timestamp, seq_id)
    }
}

const TIMESTAMP_RANGE: std::ops::Range<usize> = 22..63;
const NODE_ID_RANGE: std::ops::Range<usize> = 12..22;
const SEQ_ID_RANGE: std::ops::Range<usize> = 0..12;

/// +--------------------------------------------------------------------------+
/// | 1 Bit Unused | 41 Bit Timestamp |  10 Bit NodeID  |   12 Bit Sequence ID |
/// +--------------------------------------------------------------------------+
#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Id(pub i64);

impl Id {
    pub fn new(node_id: i32, timestamp: i64, seq_id: i32) -> Id {
        Self(
            *0i64
                .set_bits(SEQ_ID_RANGE, seq_id as i64)
                .set_bits(NODE_ID_RANGE, node_id as i64)
                .set_bits(TIMESTAMP_RANGE, timestamp as i64),
        )
    }

    pub fn node_id(&self) -> i32 {
        self.0.get_bits(NODE_ID_RANGE) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_node() {
        let node = Node::new(1).unwrap();
        let node = Node::new(2).unwrap();
    }
}