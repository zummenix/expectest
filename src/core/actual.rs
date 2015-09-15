
use core::{SourceLocation, Matcher, Join, TestResult};

/// A function that intended to replace an `expect!` macro if desired.
pub fn expect<A>(value: A) -> ActualValue<A> {
    ActualValue::new(value)
}

/// Wrapps an actual value and a location in a source code.
#[derive(Debug)]
pub struct ActualValue<A> {
    value: A,
    location: Option<SourceLocation>,
}

impl<A> ActualValue<A> {
    /// Creates new `ActualValue`.
    pub fn new(value: A) -> Self {
        ActualValue { value: value, location: None }
    }

    /// Sets new `SourceLocation`.
    pub fn location(mut self, l: SourceLocation) -> Self {
        self.location = Some(l);
        self
    }

    /// Performs assertion with "to" word. Prints a failure message and panics
    /// if an actual value does not match with an expected value.
    pub fn to<M, E>(self, matcher: M) -> TestResult
        where M: Matcher<A, E>
    {
        if !matcher.matches(&self.value) {
            let m = matcher.failure_message(Join::To, &self.value);
            TestResult::new_failure(m, self.location)
        } else {
            TestResult::new_success()
        }
    }

    /// Performs negation with "to not" words. Prints a failure message and
    /// panics if an actual value matches with an expected value.
    pub fn to_not<M, E>(self, matcher: M) -> TestResult
        where M: Matcher<A, E>
    {
        if matcher.matches(&self.value) {
            let m = matcher.failure_message(Join::ToNot, &self.value);
            TestResult::new_failure(m, self.location)
        } else {
            TestResult::new_success()
        }
    }

    /// Performs negation with "not to" words. Prints a failure message and
    /// panics if an actual value matches with an expected value.
    pub fn not_to<M, E>(self, matcher: M) -> TestResult
        where M: Matcher<A, E>
    {
        if matcher.matches(&self.value) {
            let m = matcher.failure_message(Join::NotTo, &self.value);
            TestResult::new_failure(m, self.location)
        } else {
            TestResult::new_success()
        }
    }
}
