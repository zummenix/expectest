
use core::{Matcher, Join};

/// A matcher for `be_empty` assertions.
pub struct BeEmpty;

/// Returns a new `BeEmpty` matcher.
pub fn be_empty() -> BeEmpty {
    BeEmpty
}

impl<A, T> Matcher<A, ()> for BeEmpty
where
    A: Iterator<Item = T> + Clone,
{
    fn failure_message(&self, join: Join, actual: &A) -> String {
        if join.is_assertion() {
            let count = actual.clone().count();
            format!("expected {} be empty, got the length of {}", join, count)
        } else {
            format!("expected {} be empty", join)
        }
    }

    fn matches(&self, actual: &A) -> bool {
        actual.clone().count() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::be_empty;
    use core::expect;

    #[test]
    fn be_empty_str_failure_message() {
        expect("hello".chars()).to(be_empty()).assert_eq_message(
            "expected to be empty, got the length of 5",
        );
    }

    #[test]
    fn to_not_be_empty_str_failure_message() {
        expect("".chars()).to_not(be_empty()).assert_eq_message(
            "expected to not be empty",
        );
    }
}
