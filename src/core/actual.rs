
use core::{ SourceLocation, Matcher };

pub fn expect<T>(value: T) -> ActualValue<T> {
    ActualValue::new(value)
}

pub struct ActualValue<A> {
    value: A,
    location: Option<SourceLocation>,
}

impl<A> ActualValue<A> {
    fn new(value: A) -> ActualValue<A> {
        ActualValue {
            value: value,
            location: None,
        }
    }

    pub fn location(mut self, l: SourceLocation) -> Self {
        self.location = Some(l);
        self
    }

    pub fn to<M, E>(self, matcher: M) where M: Matcher<A, E> {
        if !matcher.matches(&self.value) {
            let message = matcher.failure_message("to", &self.value);
            failure(message, self.location);
        }
    }

    pub fn to_not<M, E>(self, matcher: M) where M: Matcher<A, E> {
        if matcher.matches(&self.value) {
            let message = matcher.failure_message("to not", &self.value);
            failure(message, self.location);
        }
    }

    pub fn not_to<M, E>(self, matcher: M) where M: Matcher<A, E> {
        if matcher.matches(&self.value) {
            let message = matcher.failure_message("not to", &self.value);
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
