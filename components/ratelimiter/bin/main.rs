use ratelimiter::clock::*;
use ratelimiter::Limiter;
use ratelimiter::{Config, LeakyBucketLimiter};

use std::time::{Duration, SystemTime};

use std::thread;

fn main() {
    let conf = Config {
        clock: Box::new(DefaultClock {}),
        slack: 0,
        per_req: Duration::new(1, 0),
    };

    let limiter = LeakyBucketLimiter::new(conf);

    let mut prev = SystemTime::now();
    for _ in 0..10 {
        let t = limiter.take();
        println!("{:?}, {:?}", t, t.duration_since(prev).unwrap());
        prev = t;
    }


    prev = SystemTime::now();
    let mut threads = vec![];
    // thread spawn
    for _ in 0..10 {
        let inner_limiter = limiter.clone();
        let thread_handle = thread::spawn(move || {
            inner_limiter.take();
        });
        threads.push(thread_handle);
    }
    threads.into_iter().map(move |t | {
        t.join().unwrap();
    });
    let t = limiter.take();
    println!("{:?}, {:?}", t, t.duration_since(prev).unwrap());

}
