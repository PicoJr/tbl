use crate::Block;
use std::fmt::Debug;

const TEXT_FULL: &str = "=";
const TEXT_EMPTY: &str = " ";
/// ~ Terminal width
pub const DEFAULT_LENGTH: usize = 90; // ~ terminal width

#[derive(Debug, PartialEq, Clone)]
/// A bar is rendered as a list of `RenderBlock`.
pub enum RenderBlock {
    /// A block represents an interval.
    Block(String),
    /// A space between intervals.
    Space(String),
}

impl From<RenderBlock> for String {
    fn from(r: RenderBlock) -> Self {
        match r {
            RenderBlock::Block(s) => s,
            RenderBlock::Space(s) => s,
        }
    }
}

impl From<&RenderBlock> for String {
    fn from(r: &RenderBlock) -> Self {
        match r {
            RenderBlock::Block(s) => s.clone(),
            RenderBlock::Space(s) => s.clone(),
        }
    }
}

impl PartialEq<std::string::String> for RenderBlock {
    fn eq(&self, other: &String) -> bool {
        match self {
            RenderBlock::Block(s) => s == other,
            RenderBlock::Space(s) => s == other,
        }
    }
}

pub(crate) fn render_default<L>(b: &Block<L>) -> RenderBlock
where
    L: Clone + Debug,
{
    match b {
        Block::Space(length) => RenderBlock::Space(TEXT_EMPTY.repeat(*length)),
        Block::Segment(length, _) => RenderBlock::Block(TEXT_FULL.repeat(*length)),
    }
}

pub(crate) fn render_blocks<L: Clone + Debug>(
    blocks: &[Block<L>],
    renderer: &dyn Fn(&Block<L>) -> RenderBlock,
) -> Vec<RenderBlock> {
    blocks.iter().map(|b| renderer(b)).collect()
}
