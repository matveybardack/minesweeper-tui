use std::time::{Duration, Instant};

/// Game timer with update from server
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
        self.deadline >= now
    }
}
