
use std::fmt;
use core::{Matcher, Join};

/// A matcher for `be_ok` assertions for `Result<T, E>` type.
pub struct BeOk<E> {
    expected: Option<E>,
}

/// Returns new `BeOk` matcher.
pub fn be_ok<E>() -> BeOk<E> {
    BeOk { expected: None }
}

impl<E> BeOk<E> {
    /// Sets new `ok` value.
    pub fn value(mut self, v: E) -> BeOk<E> {
        self.expected = Some(v);
        self
    }
}

impl<A, E, T> Matcher<Result<A, T>, E> for BeOk<E>
    where
        A: PartialEq<E> + fmt::Debug,
        E: fmt::Debug,
        T: fmt::Debug {

    fn failure_message(&self, join: Join, actual: &Result<A, T>) -> String {
        format!("expected {} be Ok, got <{:?}>", join, actual)
    }

    fn matches(&self, actual: &Result<A, T>) -> bool {
        ::utils::is_some_value(actual.as_ref().ok(), self.expected.as_ref())
    }
}

/// A matcher for `be_err` assertions for `Result<T, E>` type.
pub struct BeErr<E> {
    expected: Option<E>,
}

/// Returns new `BeErr` matcher.
pub fn be_err<E>() -> BeErr<E> {
    BeErr { expected: None }
}

impl<E> BeErr<E> {
    /// Sets new `err` value.
    pub fn value(mut self, v: E) -> BeErr<E> {
        self.expected = Some(v);
        self
    }
}

impl<A, E, T> Matcher<Result<T, A>, E> for BeErr<E>
    where
        A: PartialEq<E> + fmt::Debug,
        E: fmt::Debug,
        T: fmt::Debug {

    fn failure_message(&self, join: Join, actual: &Result<T, A>) -> String {
        format!("expected {} be Err, got <{:?}>", join, actual)
    }

    fn matches(&self, actual: &Result<T, A>) -> bool {
        ::utils::is_some_value(actual.as_ref().err(), self.expected.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::{be_ok, be_err};
    use core::Matcher;

    fn ok_result(value: u32) -> Result<u32, &'static str> {
        Ok(value)
    }

    fn err_result(value: &'static str) -> Result<u32, &'static str> {
        Err(value)
    }

    #[test]
    fn be_ok_matches_ok() {
        assert!(be_ok().matches(&ok_result(5)));
    }

    #[test]
    fn be_ok_value_matches_ok() {
        assert!(be_ok().value(1).matches(&ok_result(1)));
    }

    #[test]
    #[should_panic]
    fn be_ok_matches_err_should_panic() {
        assert!(be_ok().matches(&err_result("error")));
    }

    #[test]
    #[should_panic]
    fn be_ok_value_matches_ok_should_panic() {
        assert!(be_ok().value(1).matches(&ok_result(2)));
    }

    #[test]
    #[should_panic]
    fn be_ok_value_matches_err_should_panic() {
        assert!(be_ok().value(1).matches(&err_result("error")));
    }

    #[test]
    fn be_err_matches_err() {
        assert!(be_err::<&'static str>().matches(&err_result("error")));
    }

    #[test]
    fn be_err_value_matches_err() {
        assert!(be_err().value("error").matches(&err_result("error")));
    }

    #[test]
    #[should_panic]
    fn be_err_matches_ok_should_panic() {
        assert!(be_err::<&'static str>().matches(&ok_result(1)));
    }

    #[test]
    #[should_panic]
    fn be_err_value_matches_err_should_panic() {
        assert!(be_err().value("error").matches(&err_result("error!")));
    }

    #[test]
    #[should_panic]
    fn be_err_value_matches_ok_should_panic() {
        assert!(be_err().value("error").matches(&ok_result(1)));
    }
}
