
use core::Matcher;

pub struct BeEmpty;

pub fn be_empty() -> BeEmpty {
    BeEmpty
}

impl<'a> Matcher<&'a str, bool> for BeEmpty {
    fn failure_message(&self, join: &'static str, actual: &&str) -> String {
        format!("expected {} be empty, got <{:?}>", join, actual)
    }

    fn matches(&self, actual: &&str) -> bool {
        actual.is_empty()
    }
}

impl Matcher<String, bool> for BeEmpty {
    fn failure_message(&self, join: &'static str, actual: &String) -> String {
        format!("expected {} be empty, got <{:?}>", join, actual)
    }

    fn matches(&self, actual: &String) -> bool {
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
}
