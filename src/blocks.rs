use crate::interval::{is_empty, is_finite, scale, size, space_between, translate, TBLInterval};
use crate::{Block, Bound, TBCError};
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
) -> Result<Vec<TBLBlock<L>>, TBCError>
where
    L: Clone + Debug,
{
    let intervals: Vec<&TBLInterval<L>> = intervals
        .iter()
        .filter(|interval| is_finite(interval))
        .filter(|interval| !is_empty(interval))
        .sorted()
        .collect();
    let blocks: Result<Vec<TBLBlock<L>>, TBCError> = match intervals.as_slice() {
        [] => Err(TBCError::Empty),
        [interval] => Ok(vec![TBLBlock::Segment(TBLInterval::new(
            interval.bounds,
            interval.label.clone(),
        ))]),
        _ => {
            let none_delimited = intervals.iter().map(Some).chain(iter::once(None));
            let windowed = none_delimited.tuple_windows::<(_, _)>();
            Ok(windowed
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
                .collect())
        }
    };
    let blocks = blocks?;
    let intervals_boundaries = crate::interval::boundaries(intervals.as_slice());
    let padded_blocks = if let Some((left, right)) = padding(intervals_boundaries, boundaries) {
        iter::once(TBLBlock::Space(TBLInterval::new(left, None)))
            .chain(blocks)
            .chain(iter::once(TBLBlock::Space(TBLInterval::new(right, None))))
            .collect()
    } else {
        blocks
    };
    let (min_start, max_end) = (intervals_boundaries.ok_or_else(|| TBCError::NoBoundaries))?;
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
) -> Option<((f64, f64), (f64, f64))> {
    match (intervals_boundaries, boundaries) {
        (Some((a0, b0)), Some((a1, b1))) => {
            let left = (a0.min(a1), a0.max(a1));
            let right = (b0.min(b1), b0.max(b1));
            Some((left, right))
        }
        _ => None,
    }
}
