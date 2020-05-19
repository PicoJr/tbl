use crate::blocks::build_blocks;
use crate::interval::TBLInterval;
use crate::rendering::{render_blocks, DefaultRenderer, DEFAULT_LENGTH};
use crate::{Block, BlockRenderer, Bound, TBCError};
use std::fmt::Debug;

pub struct Renderer<'a, L>
where
    L: Clone + Debug,
{
    length: usize,
    intervals: Vec<TBLInterval<L>>,
    renderer: &'a dyn BlockRenderer<L>,
    boundaries: Option<Bound>,
}

impl<'a, L> Renderer<'a, L>
where
    L: Clone + Debug,
{
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
            renderer: &DefaultRenderer {},
            boundaries: None,
        }
    }

    pub fn with_length(&'a mut self, length: usize) -> &'a mut Renderer<'a, L> {
        self.length = length;
        self
    }

    pub fn with_boundaries(&'a mut self, boundaries: Bound) -> &'a mut Renderer<'a, L> {
        self.boundaries = Some(boundaries);
        self
    }

    pub fn with_renderer(
        &'a mut self,
        renderer: &'a dyn BlockRenderer<L>,
    ) -> &'a mut Renderer<'a, L> {
        self.renderer = renderer;
        self
    }

    pub fn render(&self) -> Result<String, TBCError> {
        let blocks = build_blocks(self.intervals.as_slice(), self.length, self.boundaries)?;
        let blocks: Vec<Block<L>> = blocks.iter().map(|b| Block::from(b.clone())).collect();
        let rendered = render_blocks(blocks.as_slice(), self.renderer);
        let rendered: Vec<String> = rendered.iter().map(String::from).collect();
        Ok(rendered.concat())
    }
}
