//! # TBL
//! (T)erm (B)ar (time)Line.
//!
//! ## Example
//!
//! ```
//! use tbl::Renderer;
//!
//! let data = vec![(0., 2.), (3., 4.)];
//! let rendered = Renderer::new(data.as_slice(), &|&e| e, &|_| None::<String>) // explicit type for Option<Label>
//!     .with_length(42)
//!     .render()
//!     .unwrap();
//! assert_eq!(rendered, "=====================          ===========");
//! ```
//!
//! ## Custom Data and Renderer
//!
//! ```
//! use tbl::{Block, BlockRenderer, RenderBlock, Renderer, TBLError, Bound};
//!
//! struct CustomData {
//!    bounds: (usize, usize),
//!    label: String // must be Clone + Debug
//! }
//!
//! fn bounds(cd: &CustomData)-> Bound {
//!     let (a, b) = cd.bounds;
//!     (a as f64, b as f64)
//! }
//!
//! fn label(cd: &CustomData)-> Option<String> {
//!    Some(cd.label.clone())
//! }
//!
//! struct CustomRenderer {}
//!
//! impl BlockRenderer<String> for CustomRenderer {
//!    fn render(&self, b: &Block<String>) -> RenderBlock {
//!        match b {
//!            Block::Space(length) => RenderBlock::Space("\u{2606}".repeat(*length)),
//!            Block::Segment(length, label) => {
//!                let mut truncated = label.clone().unwrap_or_default();
//!                truncated.truncate(*length);
//!                RenderBlock::Block(format!(
//!                    "{}{}",
//!                    truncated,
//!                    "\u{2605}".repeat(*length - truncated.len())
//!                ))
//!            }
//!        }
//!    }
//! }
//!
//! let data = vec![CustomData{bounds: (0, 2), label: "hello".to_string()}, CustomData{bounds: (3, 4), label: "world!".to_string()}];
//! let rendered = Renderer::new(data.as_slice(), &bounds, &label)
//!        .with_length(60)
//!        .with_renderer(&CustomRenderer {})
//!        .render().unwrap();
//! assert_eq!(rendered, "hello★★★★★★★★★★★★★★★★★★★★★★★★★☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆world!★★★★★★★★★");
//! ```

use thiserror::Error;

mod blocks;
mod builder;
mod interval;
mod rendering;

pub(crate) const EPSILON: f64 = 0.1; // < 1/8
pub type Bound = (f64, f64);

pub use builder::Renderer;
pub use rendering::{BlockRenderer, RenderBlock};

/// Blocks are built, then rendered using a `BlockRenderer`.
pub enum Block<L>
where
    L: Clone,
{
    /// A space ie not data, with a size (characters)
    Space(usize),
    /// A segment representing data, with a size (characters) and an optional `label: L`
    Segment(usize, Option<L>),
}

#[derive(Error, Debug, PartialEq)]
pub enum TBLError {
    #[error("no boundaries")]
    NoBoundaries,
    #[error("`{0:?}` intersects `{1:?}` ")]
    Intersection(Bound, Bound),
}
