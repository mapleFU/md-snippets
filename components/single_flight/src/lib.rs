#![feature(get_mut_unchecked)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

// use crossbeam_utils::sync::WaitGroup;

pub struct SingleFlight<T> {
    single_flight: Arc<Mutex<SingleFlightImpl<T>>>,
}

unsafe impl<T> Send for SingleFlight<T> {}
unsafe impl<T> Sync for SingleFlight<T> {}

impl<T> SingleFlight<T> {
    pub fn new() -> Self {
        return SingleFlight { single_flight: Arc::new(Mutex::new(SingleFlightImpl::new())) };
    }
}

#[derive(Debug, Clone)]
struct Call<T> {
    value: Option<T>,
    // wg: WaitGroup,
    // TODO(mwish): using a conditional variable.
    wg: Arc<AtomicUsize>,
}

impl<T: Copy> Call<T> {
    fn new() -> Self {
        return Call {
            value: None,
            wg: Arc::new(AtomicUsize::new(0)),
        };
    }
}

struct SingleFlightImpl<T> {
    map: HashMap<String, Arc<Call<T>>>,
}

impl<T> SingleFlightImpl<T> {
    fn new() -> Self {
        SingleFlightImpl {map: HashMap::new()}
    }
}

impl<T: Copy + Clone> SingleFlight<T> {
    pub fn run<F>(&self, key: &str, func: F) -> T
    where
        F: Fn() -> T,
    {
        let mut sf_locked = self.single_flight.lock().unwrap();
        if let Some(v) = sf_locked.map.get(key) {
            let cloned_call = v.clone();
            drop(sf_locked);
            loop {
                if cloned_call.wg.load(Ordering::SeqCst) == 0 {
                    break;
                } else {
                    thread::yield_now();
                }
            }
            return cloned_call.value.unwrap();
        }

        let c = Call::new();
        c.wg.store(1, Ordering::SeqCst);
        let call = sf_locked.map.entry(key.to_string()).or_insert(Arc::new(c));
        let mut cloned_call = call.clone();
        drop(sf_locked);

        unsafe {
            Arc::get_mut_unchecked(&mut cloned_call).value = Some(func());
        }

        cloned_call.wg.store(0, Ordering::SeqCst);

        {
            let mut sf_locked = self.single_flight.lock().unwrap();
            let _ = sf_locked.map.remove(key).unwrap();
        }

        cloned_call.value.unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::SingleFlight;

    const RES: usize = 7;

    #[test]
    fn test_simple() {
        let g = SingleFlight::new();
        let res = g.run("key", || RES);
        assert_eq!(res, RES);
    }

    #[test]
    fn test_multiple_threads() {
        use std::time::Duration;

        use crossbeam_utils::thread;

        fn expensive_fn() -> usize {
            std::thread::sleep(Duration::new(0, 500));
            RES
        }

        let g = SingleFlight::new();
        thread::scope(|s| {
            for _ in 0..10 {
                s.spawn(|_| {
                    let res = g.run("key", expensive_fn);
                    assert_eq!(res, RES);
                });
            }
        })
        .unwrap();
    }
}