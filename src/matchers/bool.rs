
use core::{Matcher, Join};

/// A matcher for `be_true` assertions.
pub struct BeTrue;

/// Returns a new `BeTrue` matcher.
pub fn be_true() -> BeTrue {
    BeTrue
}

impl Matcher<bool, ()> for BeTrue {
    fn failure_message(&self, join: Join, _: &bool) -> String {
        format!("expected {} be true", join)
    }

    fn matches(&self, actual: &bool) -> bool {
        *actual
    }
}

/// A matcher for `be_false` assertions.
pub struct BeFalse;

/// Returns a new `BeFalse` matcher.
pub fn be_false() -> BeFalse {
    BeFalse
}

impl Matcher<bool, ()> for BeFalse {
    fn failure_message(&self, join: Join, _: &bool) -> String {
        format!("expected {} be false", join)
    }

    fn matches(&self, actual: &bool) -> bool {
        !*actual
    }
}

#[cfg(test)]
mod tests {
    use super::{be_true, be_false};
    use core::expect;

    #[test]
    fn test_be_true_message() {
        expect(1 == 0)
            .to(be_true())
            .assert_eq_message("expected to be true");
    }

    #[test]
    fn test_not_to_be_true_message() {
        expect(0 == 0)
            .not_to(be_true())
            .assert_eq_message("expected not to be true");
    }

    #[test]
    fn test_be_false_message() {
        expect(0 == 0)
            .to(be_false())
            .assert_eq_message("expected to be false");
    }

    #[test]
    fn test_not_to_be_false_message() {
        expect(0 == 1)
            .not_to(be_false())
            .assert_eq_message("expected not to be false")
    }
}
