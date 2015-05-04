
use std::fmt;
use core::{ Matcher, Join };

/// A matcher for `be_equal_to` assertions for types that conforms to
/// `PartialEq` trait.
pub struct BeEqualTo<E> {
    expected: E,
}

/// Returns new `BeEqualTo` matcher.
pub fn be_equal_to<E>(expected: E) -> BeEqualTo<E> {
    BeEqualTo {
        expected: expected,
    }
}

impl<A, E> Matcher<A, E> for BeEqualTo<E>
    where
        A: PartialEq<E> + fmt::Debug,
        E: fmt::Debug {

    fn failure_message(&self, join: Join, actual: &A) -> String {
        format!("expected {} be equal to <{:?}>, got <{:?}>",
            join, self.expected, actual)
    }

    fn matches(&self, actual: &A) -> bool {
        *actual == self.expected
    }
}

#[cfg(test)]
mod tests {
    use super::be_equal_to;
    use core::{ Matcher, Join };

    #[test]
    fn be_equal_to_one_matches_one() {
        assert!(be_equal_to(1).matches(&1));
    }

    #[test]
    fn be_equal_to_failure_message() {
        let message = be_equal_to(1).failure_message(Join::To, &2);
        assert!(message == "expected to be equal to <1>, got <2>");
    }

    #[test]
    #[should_panic]
    fn be_equal_to_one_matches_four_should_panic() {
        assert!(be_equal_to(1).matches(&4));
    }
}
