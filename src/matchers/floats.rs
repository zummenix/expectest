
use core::Matcher;
use std::fmt;
use num::{ self, Float };

pub struct CloseTo<E> {
    expected: E,
    delta: E,
}

pub fn close_to<E>(expected: E) -> CloseTo<E> where E: Float {
    CloseTo {
        expected: expected,
        delta: num::traits::cast(0.001).unwrap(),
    }
}

impl<E> Matcher<E, E> for CloseTo<E>
    where
        // A: PartialEq<E> + Float + fmt::Debug,
        E: Float + fmt::Debug {

    fn failure_message(&self, join: &'static str, actual: &E) -> String {
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
    use super::close_to;
    use core::Matcher;
    use num::Float;

    #[test]
    fn zero_matches_zero() {
        assert!(close_to(0.0).matches(&0.0_f32));
    }

    #[test]
    fn small_zero_matches_zero() {
        assert!(close_to(0.001_f32).matches(&0.0));
    }

    #[test]
    #[should_panic]
    fn big_zero_matches_zero_should_panic() {
        assert!(close_to(0.0011_f32).matches(&0.0));
    }

    #[test]
    fn infinity_matches_infinity() {
        let infinity: f32 = Float::infinity();
        assert!(close_to(infinity).matches(&Float::infinity()));
    }

    #[test]
    #[should_panic]
    fn infinity_matches_zero_should_panic() {
        let infinity: f32 = Float::infinity();
        assert!(close_to(infinity).matches(&0.0));
    }
}
