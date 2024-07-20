#[derive(Clone, Debug)]
pub struct Body {
    mass: f64,
    position: Vector2<f64>,
    velocity: Vector2<f64>,
}

impl Body {
    pub fn new(mass: f64, position: Vector2<f64>, velocity: Vector2<f64>) -> Self {
        Self { mass, position, velocity }
    }
}

