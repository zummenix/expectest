
use std::fmt;
use core::{Matcher, Join};

/// A matcher for `be_equal_to` assertions for types that conforms to
/// `PartialEq` trait.
pub struct BeEqualTo<E> {
    expected: E,
}

/// Returns new `BeEqualTo` matcher.
pub fn be_equal_to<E>(expected: E) -> BeEqualTo<E> {
    BeEqualTo { expected: expected }
}

impl<A, E> Matcher<A, E> for BeEqualTo<E>
    where
        A: PartialEq<E> + fmt::Debug,
        E: fmt::Debug {

    fn failure_message(&self, join: Join, actual: &A) -> String {
        if join.is_assertion() {
            format!("expected {} be equal to <{:?}>, got <{:?}>", join, self.expected, actual)
        } else {
            format!("expected {} be equal to <{:?}>", join, self.expected)
        }
    }

    fn matches(&self, actual: &A) -> bool {
        *actual == self.expected
    }
}

#[cfg(test)]
mod tests {
    use super::be_equal_to;
    use core::{Matcher, Join};

    #[test]
    fn be_equal_to_failure_message() {
        let m = be_equal_to(1).failure_message(Join::To, &2);
        assert!(m == "expected to be equal to <1>, got <2>");
    }

    #[test]
    fn to_not_be_equal_to_failure_message() {
        let m = be_equal_to(1).failure_message(Join::ToNot, &1);
        assert!(m == "expected to not be equal to <1>");
    }
}
