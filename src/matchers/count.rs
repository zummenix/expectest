
use core::{Matcher, Join};

/// A matcher for `have_count` assertions.
pub struct HaveCount {
    count: usize,
}

/// Returns new `HaveCount` matcher.
pub fn have_count(c: usize) -> HaveCount {
    HaveCount { count: c }
}

impl<A, T> Matcher<A, ()> for HaveCount where A: ExactSizeIterator<Item = T> {
    fn failure_message(&self, join: Join, actual: &A) -> String {
        format!("expected {} have count {}, got {}", join, self.count, actual.len())
    }

    fn matches(&self, actual: &A) -> bool {
        actual.len() == self.count
    }
}

#[cfg(test)]
mod tests {
    use super::have_count;
    use core::{Matcher, Join};

    #[test]
    fn test_have_count() {
        assert!(have_count(3).matches(&vec![0, 0, 0].iter()));
    }
}
