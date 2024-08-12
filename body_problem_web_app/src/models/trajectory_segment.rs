use nalgebra::Vector2;
use std::time::Duration;

#[derive(Clone, PartialEq)]
pub(crate) struct TrajectorySegment {
    pub(crate) positions: Vec<Vector2<f64>>,
    pub(crate) recorded_after: Duration,
}

impl TrajectorySegment {
    pub fn new(positions: Vec<Vector2<f64>>, recorded_after: Duration) -> Self {
        Self { positions, recorded_after }
    }
}
