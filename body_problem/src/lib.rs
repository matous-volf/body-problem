use std::cmp::max_by;

use nalgebra::Vector2;

const GRAVITATIONAL_CONSTANT: f64 = 6.6743e-11;
const BODY_DISTANCE_MIN: f64 = 50f64;

#[derive(Clone, Debug)]
pub struct Body {
    pub mass: f64,
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
}

impl Body {
    pub fn new(mass: f64, position: Vector2<f64>, velocity: Vector2<f64>) -> Self {
        Self { mass, position, velocity }
    }
}

pub fn simulate(bodies: &[Body], step: f64) -> Vec<Body> {
    let mut bodies_new = bodies.to_vec();

    for (index1, body1) in bodies_new.iter_mut().enumerate() {
        let force = bodies.iter().enumerate()
            .filter(|&(index2, _)| index1 != index2)
            .map(|(_, body2)| GRAVITATIONAL_CONSTANT * body1.mass * body2.mass * (body2.position - body1.position)
                / max_by((body2.position - body1.position).norm().powi(3), BODY_DISTANCE_MIN, |a: &f64, b: &f64| a.partial_cmp(b).unwrap()))
            .sum::<Vector2<f64>>();

        let acceleration = force / body1.mass;
        body1.velocity += acceleration * step;
    }

    for body in bodies_new.iter_mut() {
        body.position += body.velocity * step;
    }

    bodies_new
}
