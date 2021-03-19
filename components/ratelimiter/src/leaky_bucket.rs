use crate::{Config, Limiter};

use std::time::SystemTime;

use chrono::Duration as ChronoDuration;
use std::sync::{Arc, Mutex};

fn max_slack_time(slack_seconds: u32) -> ChronoDuration {
    ChronoDuration::seconds(slack_seconds as i64) * -1
}

#[derive(Clone)]
pub struct LeakyBucketLimiter {
    limiter: Arc<Mutex<LeakyBucketLimiterImpl>>,
}

impl LeakyBucketLimiter {
    pub fn new(conf: Config) -> Self {
        LeakyBucketLimiter {
            limiter: Arc::new(Mutex::new(LeakyBucketLimiterImpl {
                last: None,
                sleep_dur: ChronoDuration::seconds(0),
                conf: conf,
            })),
        }
    }
}

pub struct LeakyBucketLimiterImpl {
    last: Option<SystemTime>,
    sleep_dur: ChronoDuration,
    // immutable
    conf: Config,
}

impl LeakyBucketLimiterImpl {
    fn take_impl(&mut self) -> SystemTime {
        let now = self.conf.clock.now();

        if let None = self.last {
            self.last = Some(now);
            return now;
        }

        self.sleep_dur = ChronoDuration::from_std(self.conf.per_req).unwrap()
            + ChronoDuration::from_std(now.duration_since(self.last.unwrap()).unwrap()).unwrap();

        if self.sleep_dur < max_slack_time(self.conf.slack) {
            self.sleep_dur = max_slack_time(self.conf.slack);
        }

        if self.sleep_dur > ChronoDuration::seconds(0) {
            let current_sleep_duration = self.sleep_dur.to_std().unwrap();
            self.sleep_dur = ChronoDuration::seconds(0);
            self.conf.clock.sleep(current_sleep_duration);
            self.last = now.checked_add(current_sleep_duration);
        } else {
            self.last = Some(now);
        }

        // Note: it must hold the right value.
        self.last.unwrap()
    }
}

unsafe impl Sync for LeakyBucketLimiter {}
unsafe impl Send for LeakyBucketLimiter {}

impl Limiter for LeakyBucketLimiter {
    fn take(&self) -> SystemTime {
        self.limiter.lock().unwrap().take_impl()
    }
}
