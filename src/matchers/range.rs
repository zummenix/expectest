use core::{Join, Matcher};
use std::fmt;
use std::ops::RangeBounds;
use std::marker::PhantomData;

pub fn be_within_range<R, T>(range: R) -> BeInRange<R, T>
    where
        R: RangeBounds<T>,
        T: PartialOrd<T> + std::fmt::Debug {

    BeInRange { range, phantom: PhantomData}
}

pub struct BeInRange<R, T>
    where
        R: RangeBounds<T>,
        T: PartialOrd<T> + std::fmt::Debug
{
    range: R,
    phantom: PhantomData<T>,
}

impl<A, R> Matcher<A, ()> for BeInRange<R, A>
    where
        A: PartialOrd<A> + std::fmt::Debug,
        R: RangeBounds<A> + std::fmt::Debug,
{
    fn failure_message(&self, join: Join, actual: &A) -> String {
        format!("expected {} be within range <{:?}>, got <{:?}>", join, self.range, actual)
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
        use core::expect;
        expect(0).to(be_within_range(1..2))
            .assert_eq_message("expected to be within range <1..2>, got <0>");

    }
}
