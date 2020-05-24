use thiserror::Error;

mod blocks;
mod builder;
mod interval;
mod rendering;

pub(crate) const EPSILON: f64 = 0.1; // < 1/8
pub type Bound = (f64, f64);

pub use builder::Renderer;
pub use rendering::{BlockRenderer, RenderBlock};

pub enum Block<L>
where
    L: Clone,
{
    Space(usize),
    Segment(usize, Option<L>),
}

#[derive(Error, Debug)]
pub enum TBLError {
    #[error("no boundaries")]
    NoBoundaries,
    #[error("empty interval set")]
    Empty,
    #[error("`{0:?}` intersects `{1:?}` ")]
    Intersection(Bound, Bound),
}
