
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
    fn failure_message(&self, join: Join, actual: &A) -> String {
        if join.is_assertion() {
            format!("expected {} be empty, got <{:?}>", join, actual)
        } else {
            format!("expected {} be empty", join)
        }
    }

    fn matches(&self, actual: &A) -> bool {
        actual.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::be_empty;
    use core::{Matcher, Join};

    #[test]
    fn be_empty_string() {
        let s = "".to_string();
        assert!(be_empty().matches(&s));
    }

    #[test]
    fn be_empty_str() {
        assert!(be_empty().matches(&""));
    }

    #[test]
    #[should_panic]
    fn be_empty_string_should_panic() {
        let s = "tom".to_string();
        assert!(be_empty().matches(&s));
    }

    #[test]
    #[should_panic]
    fn be_empty_str_should_panic() {
        assert!(be_empty().matches(&"0"));
    }

    #[test]
    fn be_empty_str_failure_message() {
        let m = be_empty().failure_message(Join::To, &"hello");
        assert!(m == "expected to be empty, got <\"hello\">")
    }

    #[test]
    fn to_not_be_empty_str_failure_message() {
        let m = be_empty().failure_message(Join::ToNot, &"");
        assert!(m == "expected to not be empty")
    }

    #[test]
    fn be_empty_vec() {
        let v: Vec<u8> = vec![];
        assert!(be_empty().matches(&v));
    }

    #[test]
    #[should_panic]
    fn be_empty_vec_should_panic() {
        let v = vec![1, 2, 3];
        assert!(be_empty().matches(&v));
    }

    #[test]
    fn be_empty_array() {
        let v: &[u8] = &[];
        assert!(be_empty().matches(&v));
    }

    #[test]
    #[should_panic]
    fn be_empty_array_should_panic() {
        let v: &[u8] = &[1, 2, 3];
        assert!(be_empty().matches(&v));
    }
}
