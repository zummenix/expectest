
use std::fmt;
use { Matcher, MatchResult };

pub struct Equal<T> {
    expected: T,
}

pub fn equal<T>(expected: T) -> Equal<T> where T: PartialEq + fmt::Debug {
    Equal {
        expected: expected,
    }
}

impl<T: fmt::Debug> fmt::Display for Equal<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.expected.fmt(f)
    }
}

impl<T: PartialEq + fmt::Debug> Matcher<T> for Equal<T> {
    fn matches(&self, actual: T) -> MatchResult {
        if self.expected.eq(&actual) {
            Ok(())
        } else {
            Err(format!("{:?}", actual))
        }
    }
}

#[cfg(test)]
mod test {
    use { equal, Matcher };

    #[test]
    fn equality_of_ints() {
        equal(1).matches(1).unwrap();
    }
}
