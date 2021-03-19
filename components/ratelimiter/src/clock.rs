use std::time::{Duration, SystemTime};

pub trait Clock {
    fn now(&self) -> SystemTime;

    fn sleep(&self, d: Duration);
}

pub struct DefaultClock {}

impl Clock for DefaultClock {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }

    fn sleep(&self, d: Duration) {
        std::thread::sleep(d)
    }
}
