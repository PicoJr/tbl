use crate::Block;
use itertools::{repeat_n, Itertools};
use std::fmt::Debug;
use std::iter::once;

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
    /// A multi line space
    MultiLineSpace(Vec<String>),
    /// A multi line block
    MultiLineBlock(Vec<String>),
}

#[derive(Debug, Clone)]
pub(crate) enum SingleLineRenderBlock {
    Block(String),
    Space(String),
}

impl From<&SingleLineRenderBlock> for String {
    fn from(rb: &SingleLineRenderBlock) -> Self {
        match rb {
            SingleLineRenderBlock::Block(s) => s.clone(),
            SingleLineRenderBlock::Space(s) => s.clone(),
        }
    }
}

fn single_line(render_block: &RenderBlock) -> Option<SingleLineRenderBlock> {
    match render_block {
        RenderBlock::Block(s) => Some(SingleLineRenderBlock::Block(s.clone())),
        RenderBlock::Space(s) => Some(SingleLineRenderBlock::Space(s.clone())),
        _ => None,
    }
}

fn break_down(render_block: &RenderBlock) -> Vec<SingleLineRenderBlock> {
    match render_block {
        RenderBlock::Block(s) => vec![SingleLineRenderBlock::Block(s.clone())],
        RenderBlock::Space(s) => vec![SingleLineRenderBlock::Space(s.clone())],
        RenderBlock::MultiLineSpace(v) => v
            .iter()
            .map(|s| SingleLineRenderBlock::Space(s.clone()))
            .collect(),
        RenderBlock::MultiLineBlock(v) => v
            .iter()
            .map(|s| SingleLineRenderBlock::Block(s.clone()))
            .collect(),
    }
}

fn lines(render_block: &RenderBlock) -> usize {
    match render_block {
        RenderBlock::Block(_) => 1,
        RenderBlock::Space(_) => 1,
        RenderBlock::MultiLineSpace(v) => v.len(),
        RenderBlock::MultiLineBlock(v) => v.len(),
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

fn uncons<T: Clone>(v: Vec<T>) -> Option<(T, Vec<T>)> {
    match v.as_slice() {
        [] => None,
        [a, ..] => Some((a.clone(), v[1..].to_vec())),
    }
}

fn traverse<A: Clone, B>(f: &dyn Fn(A) -> Option<B>, a: Vec<A>) -> Option<Vec<B>> {
    match uncons(a) {
        None => Some(vec![]),
        Some((h, t)) => match (f(h), traverse(f, t)) {
            (Some(hh), Some(tt)) => Some(once(hh).chain(tt).collect()),
            _ => None,
        },
    }
}

fn transpose<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    match traverse(&uncons, v) {
        None => vec![],
        Some(couples) => {
            let head = couples.iter().map(|e| e.0.clone()).collect();
            let tail = transpose(couples.iter().map(|e| e.1.clone()).collect());
            once(head).chain(tail).collect()
        }
    }
}

pub(crate) fn render_blocks<L: Clone + Debug>(
    blocks: &[Block<L>],
    renderer: &dyn Fn(&Block<L>) -> RenderBlock,
) -> Vec<Vec<SingleLineRenderBlock>> {
    let rendered: Vec<(usize, RenderBlock)> = blocks
        .iter()
        .map(|b| match b {
            Block::Space(w) => (*w, renderer(b)),
            Block::Segment(w, _) => (*w, renderer(b)),
        })
        .collect();

    let nb_lines = rendered
        .iter()
        .map(|(_w, rb)| lines(rb))
        .max()
        .unwrap_or_default();

    fn pad_vertically<L: Clone + Debug>(
        render_block: &RenderBlock,
        width: usize,
        lines: usize,
        renderer: &dyn Fn(&Block<L>) -> RenderBlock,
    ) -> Vec<SingleLineRenderBlock> {
        let top = break_down(render_block);
        let space = single_line(&renderer(&Block::Space(width)))
            .unwrap_or_else(|| SingleLineRenderBlock::Space("".repeat(width)));
        let vpad = repeat_n(space, lines - top.len()).collect_vec();
        top.into_iter().chain(vpad.into_iter()).collect()
    }

    let columns: Vec<Vec<SingleLineRenderBlock>> = rendered
        .iter()
        .map(|(w, rb)| pad_vertically(rb, *w, nb_lines, &renderer))
        .collect();

    transpose(columns)
}
