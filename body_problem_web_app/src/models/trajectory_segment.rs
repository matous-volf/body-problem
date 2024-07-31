use nalgebra::Vector2;
use web_time::Instant;

#[derive(Clone, PartialEq)]
pub(crate) struct TrajectorySegment {
    pub(crate) positions: Vec<Vector2<f64>>,
    pub(crate) recorded_at: Instant,
}

impl TrajectorySegment {
    pub fn new(positions: Vec<Vector2<f64>>) -> Self {
        Self { positions, recorded_at: Instant::now() }
    }
}