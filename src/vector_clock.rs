use std::cmp::Ordering;

pub struct Cfg {
    proc: usize,
    me: usize,
}

pub struct VectorClockHandler {
    cfg: Cfg,
    clock: Vec<u64>,
}

// pub enum Event {
//     Request,
//     Receive,
//     Inner,
// }

impl VectorClockHandler {
    pub fn new(cfg: Cfg) -> Self {
        assert_ne!(cfg.proc, 0);
        let sz = cfg.proc;
        VectorClockHandler {
            cfg,
            clock: vec![0; sz],
        }
    }

    pub fn comp(&self, other: Vec<u64>) -> Option<Ordering> {
        assert_eq!(self.clock.len(), other.len());

        let mut larger = 0;
        let mut smaller = 0;

        for (c, o) in self.clock.iter().zip(&other) {
            match c.cmp(o) {
                Ordering::Less => {
                    smaller += 1;
                }
                Ordering::Equal => {}
                Ordering::Greater => {
                    larger += 1;
                }
            };
        }

        match (larger, smaller) {
            (0, 0) => Some(Ordering::Equal),
            (0, _) => Some(Ordering::Less),
            (_, 0) => Some(Ordering::Greater),
            _ => None,
        }
    }

    pub fn send_vec(&mut self) -> Vec<u64> {
        self.clock[self.cfg.me] += 1;
        self.clock.clone()
    }

    pub fn inner_event(&mut self) {
        self.clock[self.cfg.me] += 1;
    }

    pub fn receive_vec(&mut self, other: Vec<u64>) {
        self.clock[self.cfg.me] += 1;
        for (c, o) in self.clock.iter_mut().zip(other) {
            if o > *c {
                *c = o;
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn load_vec(&self) -> Vec<u64> {
        self.clock.clone()
    }
}

#[cfg(test)]
mod test {
    use crate::vector_clock::{Cfg, VectorClockHandler};
    use std::cmp::Ordering;

    #[test]
    fn test_vector_clock() {
        let mut datas = vec![];
        for i in 0..3 {
            datas.push(VectorClockHandler::new(Cfg { proc: 3, me: i }))
        }

        // c->b, b->a
        let v = datas[2].send_vec();
        datas[1].receive_vec(v);

        let v = datas[1].send_vec();
        datas[0].receive_vec(v);

        assert_eq!(datas[0].comp(datas[1].load_vec()), Some(Ordering::Greater));
        assert_eq!(datas[0].comp(datas[2].load_vec()), Some(Ordering::Greater));
        assert_eq!(datas[1].comp(datas[2].load_vec()), Some(Ordering::Greater));

        // a->b | b->c
        let v1 = datas[0].send_vec();
        let v2 = datas[1].send_vec();
        datas[1].receive_vec(v1);
        datas[2].receive_vec(v2);

        assert_eq!(datas[1].comp(datas[2].load_vec()), None);
    }
}
