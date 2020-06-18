use crate::interval::{
    intersect, is_empty, is_finite, scale, size, space_between, translate, TBLInterval,
};
use crate::{Block, Bound, TBLError};
use itertools::Itertools;
use std::fmt::Debug;
use std::iter;

#[derive(Clone)]
pub(crate) enum TBLBlock<L>
where
    L: Clone + Debug,
{
    Space(TBLInterval<L>),
    Segment(TBLInterval<L>),
}

impl<L> Default for TBLBlock<L>
where
    L: Clone + Debug,
{
    fn default() -> Self {
        TBLBlock::Space(TBLInterval::new((0., 1.), None::<L>))
    }
}

impl<L> From<TBLBlock<L>> for Block<L>
where
    L: Clone + Debug,
{
    fn from(b: TBLBlock<L>) -> Self {
        match b {
            TBLBlock::Space(interval) => Block::Space(size(&interval)),
            TBLBlock::Segment(interval) => Block::Segment(size(&interval), interval.label),
        }
    }
}

pub(crate) fn build_blocks<L>(
    intervals: &[TBLInterval<L>],
    length: usize,
    boundaries: Option<Bound>,
) -> Result<Vec<TBLBlock<L>>, TBLError<L>>
where
    L: Clone + Debug,
{
    let intervals: Vec<&TBLInterval<L>> = intervals
        .iter()
        .filter(|interval| is_finite(interval))
        .filter(|interval| !is_empty(interval))
        .sorted()
        .collect();
    if let Some(intersection) = intervals
        .iter()
        .tuple_windows::<(_, _)>()
        .find(|(left, right)| intersect(left, right))
    {
        let (left, right) = intersection;
        return Err(TBLError::Intersection(
            left.label.clone(),
            right.label.clone(),
        ));
    }
    let blocks: Vec<TBLBlock<L>> = match intervals.as_slice() {
        [] => vec![TBLBlock::default()],
        [interval] => vec![TBLBlock::Segment(TBLInterval::new(
            interval.bounds,
            interval.label.clone(),
        ))],
        _ => {
            let none_delimited = intervals.iter().map(Some).chain(iter::once(None));
            let windowed = none_delimited.tuple_windows::<(_, _)>();
            windowed
                .map(|(left, right)| match (left, right) {
                    (Some(&left_interval), Some(&right_interval)) => {
                        iter::once(TBLBlock::Segment(left_interval.clone()))
                            .chain(iter::once(TBLBlock::Space(space_between(
                                &left_interval,
                                &right_interval,
                            ))))
                            .collect()
                    }
                    (Some(&left_interval), _) => {
                        iter::once(TBLBlock::Segment(left_interval.clone())).collect()
                    }
                    _ => iter::empty().collect::<Vec<TBLBlock<L>>>(),
                })
                .flatten()
                .collect()
        }
    };
    let intervals_boundaries = crate::interval::boundaries(intervals.as_slice());
    let padded_blocks = match padding(intervals_boundaries, boundaries) {
        (Some(left), Some(right)) => iter::once(TBLBlock::Space(TBLInterval::new(left, None)))
            .chain(blocks)
            .chain(iter::once(TBLBlock::Space(TBLInterval::new(right, None))))
            .collect(),
        (Some(left), None) => iter::once(TBLBlock::Space(TBLInterval::new(left, None)))
            .chain(blocks)
            .collect(),
        (None, Some(right)) => blocks
            .into_iter()
            .chain(iter::once(TBLBlock::Space(TBLInterval::new(right, None))))
            .collect(),
        (None, None) => blocks,
    };
    let padded_blocks_boundaries = blocks_boundaries(padded_blocks.as_slice());
    let (min_start, max_end) = (padded_blocks_boundaries.ok_or_else(|| TBLError::NoBoundaries))?;
    let translation = -min_start;
    let ratio = (length as f64) / (max_end - min_start);
    let adjusted: Vec<TBLBlock<L>> = padded_blocks
        .iter()
        .map(|b| match b {
            TBLBlock::Space(interval) => TBLBlock::Space(translate(interval, translation)),
            TBLBlock::Segment(interval) => TBLBlock::Segment(translate(interval, translation)),
        })
        .map(|b| match b {
            TBLBlock::Space(interval) => TBLBlock::Space(scale(&interval, ratio)),
            TBLBlock::Segment(interval) => TBLBlock::Segment(scale(&interval, ratio)),
        })
        .collect();
    Ok(adjusted)
}

fn padding(
    intervals_boundaries: Option<Bound>,
    boundaries: Option<Bound>,
) -> (Option<Bound>, Option<Bound>) {
    match (intervals_boundaries, boundaries) {
        (Some((a0, b0)), Some((a1, b1))) => {
            let left = if a1 < a0 { Some((a1, a0)) } else { None };
            let right = if b0 < b1 { Some((b0, b1)) } else { None };
            (left, right)
        }
        _ => (None, None),
    }
}

fn blocks_boundaries<L: Clone + Debug>(blocks: &[TBLBlock<L>]) -> Option<Bound> {
    let intervals: Vec<&TBLInterval<L>> = blocks
        .iter()
        .map(|b| match b {
            TBLBlock::Space(interval) => interval,
            TBLBlock::Segment(interval) => interval,
        })
        .collect();
    crate::interval::boundaries(intervals.as_slice())
}
