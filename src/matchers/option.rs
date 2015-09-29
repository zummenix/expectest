
use std::fmt;
use core::{Matcher, Join};

/// A matcher for `be_some` assertions for `Option<T>` types.
pub struct BeSome<E> {
    expected: Option<E>,
}

/// Returns a new `BeSome` matcher.
pub fn be_some<E>() -> BeSome<E> {
    BeSome { expected: None }
}

impl<E> BeSome<E> {
    /// Sets new value.
    pub fn value(mut self, v: E) -> BeSome<E> {
        self.expected = Some(v);
        self
    }
}

impl<A, E> Matcher<Option<A>, Option<E>> for BeSome<E>
    where
        A: PartialEq<E> + fmt::Debug,
        E: fmt::Debug {

    fn failure_message(&self, join: Join, actual: &Option<A>) -> String {
        if self.expected.is_none() {
            format!("expected {} be Some, got <{:?}>", join, actual)
        } else {
            format!("expected {} be equal to <{:?}>, got <{:?}>",
                join, self.expected, actual)
        }
    }

    fn matches(&self, actual: &Option<A>) -> bool {
        ::utils::is_some_value(actual.as_ref(), self.expected.as_ref())
    }
}

/// A matcher for `be_none` assertions for `Option<T>` types.
pub struct BeNone;

/// Returns a new `BeNone` matcher.
pub fn be_none() -> BeNone {
    BeNone
}

impl<A> Matcher<Option<A>, ()> for BeNone
    where
        A: fmt::Debug {

    fn failure_message(&self, join: Join, actual: &Option<A>) -> String {
        format!("expected {} be None, got <{:?}>", join, actual)
    }

    fn matches(&self, actual: &Option<A>) -> bool {
        actual.is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::{be_some, be_none};
    use core::expect;

    #[test]
    fn be_some_failure_message() {
        expect(None::<u8>).to(be_some())
            .assert_eq_message("expected to be Some, got <None>");
    }

    #[test]
    fn be_some_value_failure_message_1() {
        expect(None::<u8>).to(be_some().value(1))
            .assert_eq_message("expected to be equal to <Some(1)>, got <None>");
    }

    #[test]
    fn be_some_value_failure_message_2() {
        expect(Some(2)).to(be_some().value(1))
            .assert_eq_message("expected to be equal to <Some(1)>, got <Some(2)>");
    }

    #[test]
    fn be_none_failure_message() {
        expect(Some(2)).to(be_none())
            .assert_eq_message("expected to be None, got <Some(2)>");
    }
}
