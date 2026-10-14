use std::time::{Duration, Instant};

/// Game timer with update from server
#[derive(Debug, Clone, Copy)]
pub struct Timer {
    deadline: Instant,
}

impl Timer {
    pub fn new_at(seconds_left: u32, now: Instant) -> Self {
        Timer {
            deadline: now + Duration::from_secs(seconds_left as u64),
        }
    }

    pub fn remaining_secs_at(&self, now: Instant) -> u32 {
        let left = self.deadline.saturating_duration_since(now);
        left.as_secs() as u32 + (left.subsec_nanos() > 0) as u32
    }

    pub fn is_expired_at(&self, now: Instant) -> bool {
        self.deadline <= now
    }

    pub fn reset_at(&mut self, seconds_left: u32, now: Instant) {
        *self = Timer::new_at(seconds_left, now);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ceil_rounds_remaining_secs() {
        let now = Instant::now();
        let timer = Timer::new_at(1, now);
        let remaining = timer.remaining_secs_at(now + Duration::from_millis(500));
        assert_eq!(remaining, 1);
    }

    #[test]
    fn memmory_safe_reset() {
        let now = Instant::now();
        let mut timer = Timer::new_at(1, now);
        timer.reset_at(3, now);

        let remaining = timer.remaining_secs_at(now);
        assert_eq!(3, remaining);
    }
}
