
use std::fmt;
use core::{Matcher, Join};
use traits::IsEmpty;

/// A matcher for `be_empty` assertions.
pub struct BeEmpty;

/// Returns new `BeEmpty` matcher.
pub fn be_empty() -> BeEmpty {
    BeEmpty
}

impl<A> Matcher<A, ()> for BeEmpty where A: IsEmpty + fmt::Debug {
    fn failure_message(&self, join: Join, actual: A) -> String {
        if join.is_assertion() {
            format!("expected {} be empty, got <{:?}>", join, actual)
        } else {
            format!("expected {} be empty", join)
        }
    }

    fn matches(&self, actual: A) -> (bool, A) {
        (actual.is_empty(), actual)
    }
}

#[cfg(test)]
mod tests {
    use super::be_empty;
    use core::{Matcher, Join};

    #[test]
    fn be_empty_str_failure_message() {
        let m = be_empty().failure_message(Join::To, "hello");
        assert!(m == "expected to be empty, got <\"hello\">")
    }

    #[test]
    fn to_not_be_empty_str_failure_message() {
        let m = be_empty().failure_message(Join::ToNot, "");
        assert!(m == "expected to not be empty")
    }
}
