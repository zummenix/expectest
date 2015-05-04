
use core::{ SourceLocation, Matcher, Join };

/// A function that intended to replace an `expect!` macro if desired.
pub fn expect<A>(value: A) -> ActualValue<A> {
    ActualValue::new(value)
}

/// Wrapps an actual value and location in a source code.
pub struct ActualValue<A> {
    value: A,
    location: Option<SourceLocation>,
}

impl<A> ActualValue<A> {
    /// Creates new `ActualValue`.
    fn new(value: A) -> ActualValue<A> {
        ActualValue {
            value: value,
            location: None,
        }
    }

    /// Sets new `SourceLocation`.
    pub fn location(mut self, l: SourceLocation) -> Self {
        self.location = Some(l);
        self
    }

    /// Performs assertion with "to" word. Prints a failure message and panics
    /// if an actual value does not match with an expected value.
    pub fn to<M, E>(self, matcher: M) where M: Matcher<A, E> {
        if !matcher.matches(&self.value) {
            let message = matcher.failure_message(Join::To, &self.value);
            failure(message, self.location);
        }
    }

    /// Performs negation with "to not" words. Prints a failure message and
    /// panics if an actual value matches with an expected value.
    pub fn to_not<M, E>(self, matcher: M) where M: Matcher<A, E> {
        if matcher.matches(&self.value) {
            let message = matcher.failure_message(Join::ToNot, &self.value);
            failure(message, self.location);
        }
    }

    /// Performs negation with "not to" words. Prints a failure message and
    /// panics if an actual value matches with an expected value.
    pub fn not_to<M, E>(self, matcher: M) where M: Matcher<A, E> {
        if matcher.matches(&self.value) {
            let message = matcher.failure_message(Join::NotTo, &self.value);
            failure(message, self.location);
        }
    }
}

pub fn failure(message: String, location: Option<SourceLocation>) {
    if let Some(l) = location {
        println!("\n{}\n{}\n", l, message);
    } else {
        println!("\n{}\n", message);
    }
    panic!("test failure");
}
