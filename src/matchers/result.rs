
use std::fmt;
use core::{ Matcher, Join };

/// A matcher for `be_ok` assertions for `Result<T, E>` type.
pub struct BeOk<E> {
    expected: Option<E>,
}

/// Returns new `BeOk` matcher.
pub fn be_ok<E>() -> BeOk<E> {
    BeOk {
        expected: None,
    }
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
        if self.expected.is_none() {
            format!("expected {} be Ok, got <{:?}>", join, actual)
        } else {
            format!("expected {} be equal to <{:?}>, got <{:?}>",
                join, self.expected, actual)
        }
    }

    fn matches(&self, actual: &Result<A, T>) -> bool {
        ::utils::is_some_value(actual.as_ref().ok(), self.expected.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::{ be_ok };
    use core::{ Matcher };

    fn ok_result(value: u32) -> Result<u32, &'static str> {
        Ok(value)
    }

    #[test]
    fn be_ok_matches_ok() {
        assert!(be_ok().matches(&ok_result(5)));
    }

    #[test]
    fn be_ok_value_matches_ok() {
        assert!(be_ok().value(1).matches(&ok_result(1)));
    }
}
