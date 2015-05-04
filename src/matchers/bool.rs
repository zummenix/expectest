
use core::{ Matcher, Join };

pub struct BeTrue;

pub fn be_true() -> BeTrue {
    BeTrue
}

impl Matcher<bool, bool> for BeTrue {
    fn failure_message(&self, join: Join, _: &bool) -> String {
        format!("expected {} be true", join)
    }

    fn matches(&self, actual: &bool) -> bool {
        *actual == true
    }
}

pub struct BeFalse;

pub fn be_false() -> BeFalse {
    BeFalse
}

impl Matcher<bool, bool> for BeFalse {
    fn failure_message(&self, join: Join, _: &bool) -> String {
        format!("expected {} be false", join)
    }

    fn matches(&self, actual: &bool) -> bool {
        *actual == false
    }
}

#[cfg(test)]
mod tests {
    use super::{ be_true, be_false };
    use core::{ Matcher, Join };

    #[test]
    fn test_be_true() {
        assert!(be_true().matches(&true));
    }

    #[test]
    fn test_be_true_message() {
        let message = be_true().failure_message(Join::To, &false);
        assert!(message == "expected to be true");
    }

    #[test]
    #[should_panic]
    fn test_be_true_should_panic() {
        assert!(be_true().matches(&false));
    }

    #[test]
    fn test_be_false() {
        assert!(be_false().matches(&false));
    }

    #[test]
    fn test_be_false_message() {
        let message = be_false().failure_message(Join::To, &true);
        assert!(message == "expected to be false");
    }

    #[test]
    #[should_panic]
    fn test_be_false_should_panic() {
        assert!(be_false().matches(&true));
    }
}
