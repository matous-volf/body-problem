use body_problem::Body;

#[derive(Clone, PartialEq)]
pub struct RenderedBody {
    pub(crate) index: usize,
    pub(crate) body: Body,
    pub(crate) potential_energy: f64,
    pub(crate) color: String,
}
