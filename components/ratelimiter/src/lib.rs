mod limiter;

pub mod clock;

pub use limiter::Limiter;

use std::time::Duration;

// TODO(mwish): adding a config builder.
#[allow(dead_code)]
pub struct Config {
    pub clock: Box<dyn clock::Clock>,
    pub slack: u32,
    pub per_req: Duration,
}

mod leaky_bucket;

#[warn(unused_imports)]
pub use leaky_bucket::LeakyBucketLimiter;
