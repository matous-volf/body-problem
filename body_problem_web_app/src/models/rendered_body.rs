use body_problem::Body;

#[derive(Clone, PartialEq)]
pub(crate) struct RenderedBody {
    pub(crate) index: usize,
    pub(crate) body: Body,
    pub(crate) potential_energy: f64,
    pub(crate) color: String,
}

impl RenderedBody {
    pub const fn new(index: usize, body: Body, color: String) -> Self {
        Self { index, body, potential_energy: 0f64, color }
    }
}
