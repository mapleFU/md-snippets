use std::collections::VecDeque;
use std::sync::{Mutex, CondVar};

pub struct BlockingQueue<T> {
    vec: Mutex<VecDeque<T>>,
    cv: CondVar<VecDeque<T>>,
}

impl<T> BlockingQueue<T> {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
