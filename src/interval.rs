use crate::{Bound, EPSILON};
use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub(crate) struct TBLInterval<L>
where
    L: Clone + Debug,
{
    pub bounds: Bound,
    pub label: Option<L>,
}

impl<L> TBLInterval<L>
where
    L: Clone + Debug,
{
    pub(crate) fn new(bounds: Bound, label: Option<L>) -> Self {
        let ordered_bounds = if bounds.1 < bounds.0 {
            (bounds.1, bounds.0)
        } else {
            bounds
        };
        TBLInterval {
            bounds: ordered_bounds,
            label,
        }
    }
}

impl<L: Clone + Debug> PartialOrd for TBLInterval<L> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.bounds.0.partial_cmp(&other.bounds.0)
    }
}

impl<L: Clone + Debug> PartialEq for TBLInterval<L> {
    fn eq(&self, other: &Self) -> bool {
        let (a0, b0) = self.bounds;
        let (a1, b1) = other.bounds;
        (a0 - a1).abs() < EPSILON && (b0 - b1).abs() < EPSILON
    }
}

impl<L: Clone + Debug> Eq for TBLInterval<L> {}

impl<L: Clone + Debug> Ord for TBLInterval<L> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eq(other) {
            Ordering::Equal
        } else {
            self.partial_cmp(other).unwrap()
        }
    }
}

pub(crate) fn is_empty<L: Clone + Debug>(interval: &TBLInterval<L>) -> bool {
    let (a, b) = interval.bounds;
    (a - b).abs() < EPSILON
}

pub(crate) fn is_finite<L: Clone + Debug>(interval: &TBLInterval<L>) -> bool {
    let (a, b) = interval.bounds;
    a.is_finite() && b.is_finite()
}

pub(crate) fn size<L: Clone + Debug>(interval: &TBLInterval<L>) -> usize {
    let (a, b) = interval.bounds;
    (b.floor() - a.floor()) as usize
}

pub(crate) fn translate<L: Clone + Debug>(
    interval: &TBLInterval<L>,
    translation: f64,
) -> TBLInterval<L> {
    let (a, b) = interval.bounds;
    TBLInterval::new((a + translation, b + translation), interval.label.clone())
}

pub(crate) fn scale<L: Clone + Debug>(interval: &TBLInterval<L>, ratio: f64) -> TBLInterval<L> {
    let (a, b) = interval.bounds;
    TBLInterval::new((a * ratio, b * ratio), interval.label.clone())
}

pub(crate) fn boundaries<L: Clone + Debug>(intervals: &[&TBLInterval<L>]) -> Option<Bound> {
    intervals
        .iter()
        .fold(None, |boundaries, b| match (boundaries, b) {
            (None, interval) => Some(interval.bounds),
            (Some((ma, mb)), interval) => {
                let (a, b) = interval.bounds;
                Some((a.min(ma), b.max(mb)))
            }
        })
}

pub(crate) fn space_between<L: Clone + Debug>(
    left: &TBLInterval<L>,
    right: &TBLInterval<L>,
) -> TBLInterval<L> {
    let (_left_a, left_b) = left.bounds;
    let (right_a, _right_b) = right.bounds;
    TBLInterval::new((left_b, right_a), None)
}
