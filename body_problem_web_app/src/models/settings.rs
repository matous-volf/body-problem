use std::time::Duration;

#[derive(Clone, PartialEq)]
pub(crate) struct Settings {
    pub(crate) trajectory_duration: Duration,
    pub(crate) simulation_speed: f64,
    pub(crate) body_circle_radius: f64,
    pub(crate) scale_body_circles_with_mass: bool,
}

impl Settings {
    pub const fn new(trajectory_duration: Duration, simulation_speed: f64, body_circle_radius: f64, scale_body_circles_with_mass: bool) -> Self {
        Self { trajectory_duration, simulation_speed, body_circle_radius, scale_body_circles_with_mass }
    }
}
