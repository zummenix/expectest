
use core::{SourceLocation, Matcher, Join, TestResult};

/// Creates a new instance of `ActualValue` using `value`.
///
/// This function intended to replace an `expect!` macro if desired.
pub fn expect<A>(value: A) -> ActualValue<A> {
    ActualValue::new(value)
}

/// Represent an actual value and optional location of a test case in a source code.
#[derive(Debug)]
pub struct ActualValue<A> {
    value: A,
    location: Option<SourceLocation>,
}

impl<A> ActualValue<A> {
    /// Creates a new instance of `ActualValue` using `value`.
    ///
    /// Also to create a new instance you can use `expect` function or `expect!` macro.
    /// Macro is better because it can save location of a test case in a source code.
    pub fn new(value: A) -> Self {
        ActualValue {
            value: value,
            location: None,
        }
    }

    /// Sets a new `SourceLocation`.
    pub fn location(mut self, l: SourceLocation) -> Self {
        self.location = Some(l);
        self
    }

    /// Performs assertion matching using `matcher`. Returns a new instance of `TestResult`.
    pub fn to<M, E>(self, matcher: M) -> TestResult
        where M: Matcher<A, E>
    {
        self.matching(matcher, Join::To)
    }

    /// Performs negation matching using `matcher`. Returns a new instance of `TestResult`.
    pub fn to_not<M, E>(self, matcher: M) -> TestResult
        where M: Matcher<A, E>
    {
        self.matching(matcher, Join::ToNot)
    }

    /// Performs negation matching using `matcher`. Returns a new instance of `TestResult`.
    pub fn not_to<M, E>(self, matcher: M) -> TestResult
        where M: Matcher<A, E>
    {
        self.matching(matcher, Join::NotTo)
    }

    /// Performs matching using `matcher` and `join`. Returns a new instance of `TestResult`.
    fn matching<M, E>(self, matcher: M, join: Join) -> TestResult
        where M: Matcher<A, E>
    {
        let success = if join.is_assertion() {
            matcher.matches(&self.value)
        } else {
            !matcher.matches(&self.value)
        };
        if success {
            TestResult::new_success()
        } else {
            let message = matcher.failure_message(join, &self.value);
            TestResult::new_failure(message, self.location)
        }
    }
}
