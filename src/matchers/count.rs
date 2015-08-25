
use core::{Matcher, Join};

/// A matcher for `have_count` assertions.
pub struct HaveCount {
    count: usize,
}

/// Returns new `HaveCount` matcher.
pub fn have_count(c: usize) -> HaveCount {
    HaveCount { count: c }
}

impl<A, T> Matcher<A, ()> for HaveCount where A: Iterator<Item = T> + Clone {
    fn failure_message(&self, join: Join, actual: &A) -> String {
        if join.is_assertion() {
            format!("expected {} have count <{}>, got <{}>", join, self.count, actual.clone().count())
        } else {
            format!("expected {} have count <{}>", join, self.count)
        }
    }

    fn matches(&self, actual: &A) -> bool {
        actual.clone().count() == self.count
    }
}

#[cfg(test)]
mod tests {
    use super::have_count;
    use core::{Matcher, Join};

    #[test]
    fn test_to_have_count_2_message() {
        let m = have_count(2).failure_message(Join::To, &"abc".chars());
        assert!(m == "expected to have count <2>, got <3>");
    }

    #[test]
    fn test_not_to_have_count_3_message() {
        let m = have_count(3).failure_message(Join::NotTo, &"abc".chars());
        assert!(m == "expected not to have count <3>");
    }
}
