use std::time::Duration;

#[derive(Clone, PartialEq)]
pub(crate) struct Settings {
    pub(crate) trajectory_duration: Duration,
}

impl Settings {
    pub fn new(trajectory_duration: Duration) -> Self {
        Self { trajectory_duration }
    }
}
