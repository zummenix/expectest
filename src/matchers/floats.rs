
use core::{Matcher, Join};
use std::fmt;
use num::{self, Float};

/// A matcher for `be_close_to` assertions for float numbers.
pub struct BeCloseTo<E> {
    expected: E,
    delta: E,
}

/// Returns new `BeCloseTo` matcher with default `delta` equal to `0.001`.
pub fn be_close_to<E>(expected: E) -> BeCloseTo<E>
    where E: Float
{
    BeCloseTo { expected: expected, delta: num::traits::cast(0.001).unwrap() }
}

impl<E> BeCloseTo<E> {
    /// Sets new `delta` value.
    pub fn delta(mut self, v: E) -> BeCloseTo<E> {
        self.delta = v;
        self
    }
}

impl<E> Matcher<E, E> for BeCloseTo<E> where E: Float + fmt::Debug {
    fn failure_message(&self, join: Join, actual: &E) -> String {
        format!("expected {} be close to <{:?}> ±{:?}, got <{:?}>",
            join, self.expected, self.delta, actual)
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
    use core::{Matcher, Join};

    #[test]
    fn close_to_one_failure_message() {
        let m = be_close_to(1.0_f32).failure_message(Join::To, &0.0);
        assert!(m == "expected to be close to <1> ±0.001, got <0>");
    }

    #[test]
    fn to_not_be_close_to_one_failure_message() {
        let m = be_close_to(1.0_f32).failure_message(Join::NotTo, &0.999);
        assert!(m == "expected not to be close to <1> ±0.001, got <0.999>");
    }

    #[test]
    fn close_to_one_delta_failure_message() {
        let m = be_close_to(1.0_f32).delta(0.1).failure_message(Join::To, &0.0);
        assert!(m == "expected to be close to <1> ±0.1, got <0>");
    }
}
