use std::fmt;
use core::{Join, Matcher};

/// A matcher for `be_ok` assertions for `Result<T, E>` type.
pub struct BeOk;

/// Returns a new `BeOk` matcher.
pub fn be_ok() -> BeOk {
    BeOk
}

impl BeOk {
    /// Returns a new `BeOkValue` matcher.
    pub fn value<E>(self, v: E) -> BeOkValue<E> {
        BeOkValue { value: v }
    }
}

impl<A, T> Matcher<Result<A, T>, ()> for BeOk
where
    A: fmt::Debug,
    T: fmt::Debug,
{
    fn failure_message(&self, join: Join, actual: &Result<A, T>) -> String {
        format!("expected {} be <Ok>, got <{:?}>", join, actual)
    }

    fn matches(&self, actual: &Result<A, T>) -> bool {
        actual.is_ok()
    }
}

/// A matcher for `be_ok` assertions with value for `Result<T, E>` type.
pub struct BeOkValue<E> {
    value: E,
}

impl<A, E, T> Matcher<Result<A, T>, E> for BeOkValue<E>
where
    A: PartialEq<E> + fmt::Debug,
    E: fmt::Debug,
    T: fmt::Debug,
{
    fn failure_message(&self, join: Join, actual: &Result<A, T>) -> String {
        if join.is_assertion() {
            format!(
                "expected {} be <Ok({:?})>, got <{:?}>",
                join, self.value, actual
            )
        } else {
            format!("expected {} be <{:?}>", join, actual)
        }
    }

    fn matches(&self, actual: &Result<A, T>) -> bool {
        if let Ok(a) = actual.as_ref() {
            a == &self.value
        } else {
            false
        }
    }
}

/// A matcher for `be_err` assertions for `Result<T, E>` type.
pub struct BeErr;

/// Returns a new `BeErr` matcher.
pub fn be_err() -> BeErr {
    BeErr
}

impl BeErr {
    /// Returns a new `BeErrValue` matcher.
    pub fn value<E>(self, v: E) -> BeErrValue<E> {
        BeErrValue { value: v }
    }
}

impl<A, T> Matcher<Result<T, A>, ()> for BeErr
where
    A: fmt::Debug,
    T: fmt::Debug,
{
    fn failure_message(&self, join: Join, actual: &Result<T, A>) -> String {
        format!("expected {} be <Err>, got <{:?}>", join, actual)
    }

    fn matches(&self, actual: &Result<T, A>) -> bool {
        actual.is_err()
    }
}

/// A matcher for `be_err` assertions with value for `Result<T, E>` type.
pub struct BeErrValue<E> {
    value: E,
}

impl<A, E, T> Matcher<Result<T, A>, E> for BeErrValue<E>
where
    A: PartialEq<E> + fmt::Debug,
    E: fmt::Debug,
    T: fmt::Debug,
{
    fn failure_message(&self, join: Join, actual: &Result<T, A>) -> String {
        if join.is_assertion() {
            format!(
                "expected {} be <Err({:?})>, got <{:?}>",
                join, self.value, actual
            )
        } else {
            format!("expected {} be <{:?}>", join, actual)
        }
    }

    fn matches(&self, actual: &Result<T, A>) -> bool {
        if let Err(a) = actual.as_ref() {
            a == &self.value
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{be_err, be_ok};
    use core::expect;

    fn ok_result(value: u32) -> Result<u32, &'static str> {
        Ok(value)
    }

    fn err_result(value: &'static str) -> Result<u32, &'static str> {
        Err(value)
    }

    #[test]
    fn be_ok_failure_message() {
        expect(err_result("error"))
            .to(be_ok())
            .assert_eq_message("expected to be <Ok>, got <Err(\"error\")>");
    }

    #[test]
    fn be_ok_value_failure_message() {
        expect(err_result("error"))
            .to(be_ok().value(5))
            .assert_eq_message("expected to be <Ok(5)>, got <Err(\"error\")>");
    }

    #[test]
    fn to_not_be_ok_value_failure_message() {
        expect(ok_result(5))
            .to_not(be_ok().value(5))
            .assert_eq_message("expected to not be <Ok(5)>");
    }

    #[test]
    fn to_not_be_ok_failure_message() {
        expect(ok_result(5))
            .to_not(be_ok())
            .assert_eq_message("expected to not be <Ok>, got <Ok(5)>");
    }

    #[test]
    fn be_err_failure_message() {
        expect(ok_result(2))
            .to(be_err())
            .assert_eq_message("expected to be <Err>, got <Ok(2)>");
    }

    #[test]
    fn be_err_value_failure_message() {
        expect(ok_result(2))
            .to(be_err().value("error"))
            .assert_eq_message("expected to be <Err(\"error\")>, got <Ok(2)>");
    }

    #[test]
    fn to_not_be_err_value_failure_message() {
        expect(err_result("error"))
            .to_not(be_err().value("error"))
            .assert_eq_message("expected to not be <Err(\"error\")>");
    }

    #[test]
    fn to_not_be_err_failure_message() {
        expect(err_result("error"))
            .to_not(be_err())
            .assert_eq_message("expected to not be <Err>, got <Err(\"error\")>");
    }
}
