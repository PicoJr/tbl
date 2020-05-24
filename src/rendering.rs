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

/// A custom BlockRenderer is provided to a `Renderer` with `Renderer.with_renderer`.
///
/// ```
/// use tbl::{Block, BlockRenderer, RenderBlock};
/// struct CustomRenderer {}
///
/// impl BlockRenderer<String> for CustomRenderer {
///     fn render(&self, b: &Block<String>) -> RenderBlock {
///         match b {
///             Block::Space(length) => RenderBlock::Space("\u{2606}".repeat(*length)),
///             Block::Segment(length, label) => {
///                 let mut truncated = label.clone().unwrap_or_default();
///                 truncated.truncate(*length);
///                 RenderBlock::Block(format!(
///                     "{}{}",
///                     truncated,
///                     "\u{2605}".repeat(*length - truncated.len())
///                 ))
///             }
///         }
///     }
/// }
///
/// let custom_renderer = CustomRenderer {};
/// assert_eq!(
///     custom_renderer.render(&Block::Space(2)),
///     RenderBlock::Space("\u{2606}\u{2606}".to_string())
/// );
/// assert_eq!(
///     custom_renderer.render(&Block::Segment(2, Some("a".to_string()))),
///     RenderBlock::Block("a\u{2605}".to_string())
/// );
/// ```
pub trait BlockRenderer<L>
where
    L: Clone + Debug,
{
    fn render(&self, b: &Block<L>) -> RenderBlock;
}

pub(crate) struct DefaultRenderer {}

impl<L: Clone + Debug> BlockRenderer<L> for DefaultRenderer {
    fn render(&self, b: &Block<L>) -> RenderBlock {
        match b {
            Block::Space(length) => RenderBlock::Space(TEXT_EMPTY.repeat(*length)),
            Block::Segment(length, _) => RenderBlock::Block(TEXT_FULL.repeat(*length)),
        }
    }
}

pub(crate) fn render_blocks<L: Clone + Debug>(
    blocks: &[Block<L>],
    renderer: &dyn BlockRenderer<L>,
) -> Vec<RenderBlock> {
    blocks.iter().map(|b| renderer.render(b)).collect()
}
