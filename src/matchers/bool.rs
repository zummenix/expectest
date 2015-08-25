
use core::{Matcher, Join};

/// A matcher for `be_true` assertions.
pub struct BeTrue;

/// Returns new `BeTrue` matcher.
pub fn be_true() -> BeTrue {
    BeTrue
}

impl Matcher<bool, ()> for BeTrue {
    fn failure_message(&self, join: Join, _: bool) -> String {
        format!("expected {} be true", join)
    }

    fn matches(&self, actual: bool) -> (bool, bool) {
        (actual == true, actual)
    }
}

/// A matcher for `be_false` assertions.
pub struct BeFalse;

/// Returns new `BeFalse` matcher.
pub fn be_false() -> BeFalse {
    BeFalse
}

impl Matcher<bool, ()> for BeFalse {
    fn failure_message(&self, join: Join, _: bool) -> String {
        format!("expected {} be false", join)
    }

    fn matches(&self, actual: bool) -> (bool, bool) {
        (actual == false, actual)
    }
}

#[cfg(test)]
mod tests {
    use super::{be_true, be_false};
    use core::{Matcher, Join};

    #[test]
    fn test_be_true_message() {
        let m = be_true().failure_message(Join::To, false);
        assert!(m == "expected to be true");
    }

    #[test]
    fn test_be_false_message() {
        let m = be_false().failure_message(Join::To, true);
        assert!(m == "expected to be false");
    }
}
