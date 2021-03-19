use std::time::SystemTime;

pub trait Limiter {
    fn take(&self) -> SystemTime;
}
