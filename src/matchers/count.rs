use crate::core::{Join, Matcher};

/// A matcher for `have_count` assertions.
pub struct HaveCount {
    count: usize,
}

/// Returns a new `HaveCount` matcher.
pub fn have_count(c: usize) -> HaveCount {
    HaveCount { count: c }
}

impl<A, T> Matcher<A, ()> for HaveCount
where
    A: Iterator<Item = T> + Clone,
{
    fn failure_message(&self, join: Join, actual: &A) -> String {
        if join.is_assertion() {
            format!(
                "expected {} have count <{}>, got <{}>",
                join,
                self.count,
                actual.clone().count()
            )
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
    use crate::core::expect;

    #[test]
    fn test_to_have_count_2_message() {
        expect("abc".chars())
            .to(have_count(2))
            .assert_eq_message("expected to have count <2>, got <3>");
    }

    #[test]
    fn test_not_to_have_count_3_message() {
        expect("abc".chars())
            .not_to(have_count(3))
            .assert_eq_message("expected not to have count <3>")
    }
}
