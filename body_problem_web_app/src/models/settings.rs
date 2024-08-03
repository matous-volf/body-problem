use std::time::Duration;

#[derive(Clone, PartialEq)]
pub(crate) struct Settings {
    pub(crate) trajectory_duration: Duration,
    pub(crate) simulation_speed: f64,
}

impl Settings {
    pub const fn new(trajectory_duration: Duration, simulation_speed: f64) -> Self {
        Self { trajectory_duration, simulation_speed }
    }
}
