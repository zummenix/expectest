
use core::Matcher;

pub struct True;

pub fn be_true() -> True {
    True
}

pub struct False;

pub fn be_false() -> False {
    False
}

impl Matcher<bool, bool> for True {
    fn format_message(&self, join: &'static str, _: &bool) -> String {
        format!("expected {} be true", join)
    }

    fn matches(&self, actual: &bool) -> bool {
        *actual == true
    }
}

impl Matcher<bool, bool> for False {
    fn format_message(&self, join: &'static str, _: &bool) -> String {
        format!("expected {} be false", join)
    }

    fn matches(&self, actual: &bool) -> bool {
        *actual == false
    }
}

#[cfg(test)]
mod test {
    use super::{ be_true, be_false };
    use core::Matcher;

    #[test]
    fn test_be_true() {
        assert!(be_true().matches(&true));
    }

    #[test]
    fn test_be_true_message() {
        let message = be_true().format_message("to", &false);
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
        let message = be_false().format_message("to", &true);
        assert!(message == "expected to be false");
    }

    #[test]
    #[should_panic]
    fn test_be_false_should_panic() {
        assert!(be_false().matches(&true));
    }
}
