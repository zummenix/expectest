
use std::fmt;
use core::{ Matcher, Join };

pub struct BeEmpty;

pub fn be_empty() -> BeEmpty {
    BeEmpty
}

impl<'a> Matcher<&'a str, bool> for BeEmpty {
    fn failure_message(&self, join: Join, actual: &&str) -> String {
        format!("expected {} be empty, got <{:?}>", join, actual)
    }

    fn matches(&self, actual: &&str) -> bool {
        actual.is_empty()
    }
}

impl Matcher<String, bool> for BeEmpty {
    fn failure_message(&self, join: Join, actual: &String) -> String {
        format!("expected {} be empty, got <{:?}>", join, actual)
    }

    fn matches(&self, actual: &String) -> bool {
        actual.is_empty()
    }
}

impl<T> Matcher<Vec<T>, bool> for BeEmpty where T: fmt::Debug {
    fn failure_message(&self, join: Join, actual: &Vec<T>) -> String {
        format!("expected {} be empty, got <{:?}>", join, actual)
    }

    fn matches(&self, actual: &Vec<T>) -> bool {
        actual.is_empty()
    }
}


impl<'a, T> Matcher<&'a [T], bool> for BeEmpty where T: fmt::Debug {
    fn failure_message(&self, join: Join, actual: &&[T]) -> String {
        format!("expected {} be empty, got <{:?}>", join, actual)
    }

    fn matches(&self, actual: &&[T]) -> bool {
        actual.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::be_empty;
    use core::Matcher;

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
