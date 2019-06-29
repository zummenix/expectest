use crate::core::{Join, Matcher};
use std::fmt;
use std::marker::PhantomData;
use std::ops::RangeBounds;

/// Returns a new `BeWithinRange` matcher.
pub fn be_within_range<R, T>(range: R) -> BeWithinRange<R, T>
where
    R: RangeBounds<T>,
    T: PartialOrd<T> + fmt::Debug,
{
    BeWithinRange {
        range,
        phantom: PhantomData,
    }
}

/// A matcher for `be_within_range` assertions.
pub struct BeWithinRange<R, T>
where
    R: RangeBounds<T>,
    T: PartialOrd<T> + fmt::Debug,
{
    range: R,
    phantom: PhantomData<T>,
}

impl<A, R> Matcher<A, ()> for BeWithinRange<R, A>
where
    A: PartialOrd<A> + fmt::Debug,
    R: RangeBounds<A> + fmt::Debug,
{
    fn failure_message(&self, join: Join, actual: &A) -> String {
        format!(
            "expected {} be within range <{:?}>, got <{:?}>",
            join, self.range, actual
        )
    }

    fn matches(&self, actual: &A) -> bool {
        use std::ops::Bound::*;

        let start_check = match self.range.start_bound() {
            Included(v) => actual >= v,
            Excluded(v) => actual > v,
            Unbounded => true,
        };

        let end_check = match self.range.end_bound() {
            Included(v) => actual <= v,
            Excluded(v) => actual < v,
            Unbounded => true,
        };

        let is_not_nan = actual.partial_cmp(actual).is_some();

        start_check && end_check && is_not_nan
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_range_message() {
        use crate::core::expect;
        expect(0)
            .to(be_within_range(1..2))
            .assert_eq_message("expected to be within range <1..2>, got <0>");
    }
}
