
use std::fmt;
use core::{Matcher, Join};

/// A matcher for `be_some` assertions for `Option<T>` types.
pub struct BeSome<E> {
    expected: Option<E>,
}

/// Returns new `BeSome` matcher.
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

    fn failure_message(&self, join: Join, actual: Option<A>) -> String {
        if self.expected.is_none() {
            format!("expected {} be Some, got <{:?}>", join, actual)
        } else {
            format!("expected {} be equal to <{:?}>, got <{:?}>",
                join, self.expected, actual)
        }
    }

    fn matches(&self, actual: Option<A>) -> (bool, Option<A>) {
        (::utils::is_some_value(actual.as_ref(), self.expected.as_ref()), actual)
    }
}

/// A matcher for `be_none` assertions for `Option<T>` types.
pub struct BeNone;

/// Returns new `BeNone` matcher.
pub fn be_none() -> BeNone {
    BeNone
}

impl<A> Matcher<Option<A>, ()> for BeNone
    where
        A: fmt::Debug {

    fn failure_message(&self, join: Join, actual: Option<A>) -> String {
        format!("expected {} be None, got <{:?}>", join, actual)
    }

    fn matches(&self, actual: Option<A>) -> (bool, Option<A>) {
        (actual.is_none(), actual)
    }
}

#[cfg(test)]
mod tests {
    use super::{be_some, be_none};
    use core::{Matcher, Join};

    #[test]
    fn be_some_failure_message() {
        let m = be_some().failure_message(Join::To, None::<u8>);
        assert!(m == "expected to be Some, got <None>");
    }

    #[test]
    fn be_some_value_failure_message() {
        let m = be_some().value(1).failure_message(Join::To, None::<u8>);
        assert!(m == "expected to be equal to <Some(1)>, got <None>");
    }

    #[test]
    fn be_none_failure_message() {
        let m = be_none().failure_message(Join::To, Some(2));
        assert!(m == "expected to be None, got <Some(2)>");
    }
}
