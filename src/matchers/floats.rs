use crate::core::{Join, Matcher};
use num_traits::{self, Float};
use std::fmt;

/// A matcher for `be_close_to` assertions for float numbers.
pub struct BeCloseTo<E> {
    expected: E,
    delta: E,
}

/// Returns a new `BeCloseTo` matcher with default `delta` equal to `0.001`.
pub fn be_close_to<E>(expected: E) -> BeCloseTo<E>
where
    E: Float,
{
    BeCloseTo {
        expected: expected,
        delta: num_traits::cast(0.001).unwrap(),
    }
}

impl<E> BeCloseTo<E> {
    /// Sets new `delta` value.
    pub fn delta(mut self, v: E) -> BeCloseTo<E> {
        self.delta = v;
        self
    }
}

impl<E> Matcher<E, E> for BeCloseTo<E>
where
    E: Float + fmt::Debug,
{
    fn failure_message(&self, join: Join, actual: &E) -> String {
        format!(
            "expected {} be close to <{:?}> ±{:?}, got <{:?}>",
            join, self.expected, self.delta, actual
        )
    }

    fn matches(&self, actual: &E) -> bool {
        if *actual == self.expected {
            true
        } else {
            (self.expected - *actual).abs() - self.delta <= E::zero()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::be_close_to;
    use crate::core::expect;

    #[test]
    fn close_to_one_and_half_failure_message() {
        expect(0.5)
            .to(be_close_to(1.5))
            .assert_eq_message("expected to be close to <1.5> ±0.001, got <0.5>");
    }

    #[test]
    fn to_not_be_close_to_one_and_half_failure_message() {
        expect(1.4991)
            .not_to(be_close_to(1.5))
            .assert_eq_message("expected not to be close to <1.5> ±0.001, got <1.4991>");
    }

    #[test]
    fn close_to_one_and_half_delta_failure_message() {
        expect(0.5)
            .to(be_close_to(1.5).delta(0.1))
            .assert_eq_message("expected to be close to <1.5> ±0.1, got <0.5>");
    }
}
