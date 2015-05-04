
use std::fmt;
use core::{ Matcher, Join };

pub struct BeLessThan<E> {
    expected: E,
}

pub fn be_less_than<E>(expected: E) -> BeLessThan<E> {
    BeLessThan {
        expected: expected,
    }
}

impl<A, E> Matcher<A, E> for BeLessThan<E>
    where
        A: PartialOrd<E> + fmt::Debug,
        E: fmt::Debug {

    fn failure_message(&self, join: Join, actual: &A) -> String {
        format!("expected {} be less than <{:?}>, got <{:?}>",
            join, self.expected, actual)
    }

    fn matches(&self, actual: &A) -> bool {
        *actual < self.expected
    }
}


pub struct BeLessOrEqualTo<E> {
    expected: E,
}

pub fn be_less_or_equal_to<E>(expected: E) -> BeLessOrEqualTo<E> {
    BeLessOrEqualTo {
        expected: expected,
    }
}

impl<A, E> Matcher<A, E> for BeLessOrEqualTo<E>
    where
        A: PartialOrd<E> + fmt::Debug,
        E: fmt::Debug {

    fn failure_message(&self, join: Join, actual: &A) -> String {
        format!("expected {} be less or equal to <{:?}>, got <{:?}>",
            join, self.expected, actual)
    }

    fn matches(&self, actual: &A) -> bool {
        *actual <= self.expected
    }
}


pub struct BeGreaterThan<E> {
    expected: E,
}

pub fn be_greater_than<E>(expected: E) -> BeGreaterThan<E> {
    BeGreaterThan {
        expected: expected,
    }
}

impl<A, E> Matcher<A, E> for BeGreaterThan<E>
    where
        A: PartialOrd<E> + fmt::Debug,
        E: fmt::Debug {

    fn failure_message(&self, join: Join, actual: &A) -> String {
        format!("expected {} be greater than <{:?}>, got <{:?}>",
            join, self.expected, actual)
    }

    fn matches(&self, actual: &A) -> bool {
        *actual > self.expected
    }
}


pub struct BeGreaterOrEqualTo<E> {
    expected: E,
}

pub fn be_greater_or_equal_to<E>(expected: E) -> BeGreaterOrEqualTo<E> {
    BeGreaterOrEqualTo {
        expected: expected,
    }
}

impl<A, E> Matcher<A, E> for BeGreaterOrEqualTo<E>
    where
        A: PartialOrd<E> + fmt::Debug,
        E: fmt::Debug {

    fn failure_message(&self, join: Join, actual: &A) -> String {
        format!("expected {} be greater or equal to <{:?}>, got <{:?}>",
            join, self.expected, actual)
    }

    fn matches(&self, actual: &A) -> bool {
        *actual >= self.expected
    }
}

#[cfg(test)]
mod tests {
    use super::{
        be_less_than, be_less_or_equal_to,
        be_greater_than, be_greater_or_equal_to,
    };
    use core::{ Matcher, Join };

    #[test]
    fn be_less_than_one_matches() {
        assert!(be_less_than(1).matches(&0));
    }

    #[test]
    #[should_panic]
    fn be_less_than_one_should_panic() {
        assert!(be_less_than(1).matches(&1));
    }

    #[test]
    fn be_less_than_one_failure_message() {
        let message = be_less_than(1).failure_message(Join::To, &1);
        assert!(message == "expected to be less than <1>, got <1>");
    }

    #[test]
    fn be_less_or_equal_to_one_matches() {
        assert!(be_less_or_equal_to(1).matches(&1));
    }

    #[test]
    fn be_less_or_equal_to_one_matches_zero() {
        assert!(be_less_or_equal_to(1).matches(&0));
    }

    #[test]
    #[should_panic]
    fn be_less_or_equal_to_one_should_panic() {
        assert!(be_less_or_equal_to(1).matches(&2));
    }

    #[test]
    fn be_less_or_equal_to_one_failure_message() {
        let message = be_less_or_equal_to(1).failure_message(Join::To, &2);
        assert!(message == "expected to be less or equal to <1>, got <2>");
    }

    #[test]
    fn be_greater_than_zero_matches() {
        assert!(be_greater_than(0).matches(&1));
    }

    #[test]
    #[should_panic]
    fn be_greater_than_zero_should_panic() {
        assert!(be_greater_than(0).matches(&0));
    }

    #[test]
    fn be_greater_than_zero_failure_message() {
        let message = be_greater_than(0).failure_message(Join::To, &0);
        assert!(message == "expected to be greater than <0>, got <0>");
    }

    #[test]
    fn be_greater_or_equal_to_zero_matches() {
        assert!(be_greater_or_equal_to(0).matches(&0));
    }

    #[test]
    fn be_greater_or_equal_to_zero_matches_one() {
        assert!(be_greater_or_equal_to(0).matches(&1));
    }

    #[test]
    #[should_panic]
    fn be_greater_or_equal_to_zero_should_panic() {
        assert!(be_greater_or_equal_to(0).matches(&-1));
    }

    #[test]
    fn be_greater_or_equal_to_zero_failure_message() {
        let message = be_greater_or_equal_to(0).failure_message(Join::To, &-1);
        assert!(message == "expected to be greater or equal to <0>, got <-1>");
    }
}
