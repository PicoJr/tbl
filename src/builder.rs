//! A Renderer builds `Blocks` from provided intervals and render them.

use crate::blocks::build_blocks;
use crate::interval::TBLInterval;
use crate::rendering::{render_blocks, render_default, DEFAULT_LENGTH};
use crate::{Block, Bound, RenderBlock, TBLError};
use std::fmt::Debug;

/// Render intervals.
///
/// L is the type of labels used by the `BlockRenderer`.
pub struct Renderer<'a, L>
where
    L: Clone + Debug,
{
    length: usize,
    intervals: Vec<TBLInterval<L>>,
    renderer: &'a dyn Fn(&Block<L>) -> RenderBlock,
    boundaries: Option<Bound>,
}

impl<'a, L> Renderer<'a, L>
where
    L: Clone + Debug,
{
    /// Returns a `Renderer` with default length and default `BlockRenderer<_>`.
    ///
    /// `fb` returns a `Bound` for an interval of type `T`
    ///
    /// `fl` returns an optional `label: L` for an interval of type `T`
    ///
    /// ```
    /// use tbl::{Bound, Renderer};
    /// let data: Vec<Bound> = vec![(1., 2.), (3., 4.)]; // T = (f64, f64)
    /// let rendered = Renderer::new(data.as_slice(), &|&e| e, &|_| None::<String>) // L = String
    ///     .with_length(6)
    ///     .render();
    /// for line in rendered.unwrap() {
    ///     assert_eq!(line, "==  ==");
    /// }
    /// ```
    pub fn new<T>(
        intervals: &[T],
        fb: &dyn Fn(&T) -> Bound,
        fl: &dyn Fn(&T) -> Option<L>,
    ) -> Renderer<'a, L> {
        Renderer {
            length: DEFAULT_LENGTH,
            intervals: intervals
                .iter()
                .map(|interval| TBLInterval::new(fb(interval), fl(interval)))
                .collect(),
            renderer: &render_default,
            boundaries: None,
        }
    }

    /// Configure `Renderer` length (output line length)
    ///
    /// ```
    /// use tbl::{Bound, Renderer};
    /// let data: Vec<Bound> = vec![(1., 2.)];
    /// let rendered = Renderer::new(data.as_slice(), &|&e| e, &|_| None::<String>)
    ///     .with_length(6)
    ///     .render();
    /// for line in rendered.unwrap() {
    ///     assert_eq!(line, "======");
    /// }
    /// let rendered = Renderer::new(data.as_slice(), &|&e| e, &|_| None::<String>)
    ///     .with_length(8)
    ///     .render();
    /// for line in rendered.unwrap() {
    ///     assert_eq!(line, "========");
    /// }
    /// ```
    pub fn with_length(&'a mut self, length: usize) -> &'a mut Renderer<'a, L> {
        self.length = length;
        self
    }

    /// Configure `Renderer` intervals boundaries.
    ///
    /// if provided boundaries is not a sub-boundary of provided intervals then
    /// output is padded left and right (if necessary) to fit the provided boundary.
    ///
    /// ```
    /// use tbl::{Bound, Renderer};
    /// let data: Vec<Bound> = vec![(1., 2.), (3., 4.)];
    /// let rendered = Renderer::new(data.as_slice(), &|&e| e, &|_| None::<String>)
    ///     .with_length(6)
    ///     .render();
    /// for line in rendered.unwrap() {
    ///     assert_eq!(line, "==  ==");
    /// }
    /// let rendered = Renderer::new(data.as_slice(), &|&e| e, &|_| None::<String>)
    ///     .with_length(10)
    ///     .with_boundaries((0., 5.))
    ///     .render();
    /// for line in rendered.unwrap() {
    ///     assert_eq!(line, "  ==  ==  ");
    /// }
    /// ```
    pub fn with_boundaries(&'a mut self, boundaries: Bound) -> &'a mut Renderer<'a, L> {
        self.boundaries = Some(boundaries);
        self
    }

    /// Provide a custom renderer
    ///
    /// ```
    /// use tbl::{Block, RenderBlock, Bound, Renderer};
    ///
    /// fn render(b: &Block<String>) -> RenderBlock {
    ///    match b {
    ///        Block::Space(length) => RenderBlock::Space("\u{2606}".repeat(*length)),
    ///        Block::Segment(length, label) => {
    ///            let mut truncated = label.clone().unwrap_or_default();
    ///            truncated.truncate(*length);
    ///            RenderBlock::Block(format!(
    ///                "{}{}",
    ///                truncated,
    ///                "\u{2605}".repeat(*length - truncated.len())
    ///            ))
    ///        }
    ///    }
    /// }
    /// let data: Vec<Bound> = vec![(1., 2.), (3., 4.)];
    /// let rendered = Renderer::new(data.as_slice(), &|&e| e, &|e| {
    ///         Some(format!("{:?}", e))
    /// })
    /// .with_length(60)
    /// .with_renderer(&render)
    /// .render();
    /// for line in rendered.unwrap() {
    ///     assert_eq!(line, "(1.0, 2.0)★★★★★★★★★★☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆☆(3.0, 4.0)★★★★★★★★★★");
    /// }
    /// ```
    pub fn with_renderer(
        &'a mut self,
        renderer: &'a dyn Fn(&Block<L>) -> RenderBlock,
    ) -> &'a mut Renderer<'a, L> {
        self.renderer = renderer;
        self
    }

    /// Render intervals as a `String`.
    ///
    /// ```
    /// use tbl::{Bound, Renderer};
    /// let data: Vec<Bound> = vec![(1., 2.), (3., 4.)]; // T = (f64, f64)
    /// let rendered = Renderer::new(data.as_slice(), &|&e| e, &|_| None::<String>) // L = String
    ///     .with_length(6)
    ///     .render();
    /// for line in rendered.unwrap() {
    ///     assert_eq!(line, "==  ==");
    /// }
    /// ```
    pub fn render(&self) -> Result<Vec<String>, TBLError> {
        let blocks = build_blocks(self.intervals.as_slice(), self.length, self.boundaries)?;
        let blocks: Vec<Block<L>> = blocks.iter().map(|b| Block::from(b.clone())).collect();
        let rendered = render_blocks(blocks.as_slice(), self.renderer);
        let rendered: Vec<String> = rendered
            .iter()
            .map(|v| v.iter().map(|rb| String::from(rb)).collect())
            .collect();
        Ok(rendered)
    }
}
