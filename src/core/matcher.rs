
use std::fmt;

pub type MatchResult = Result<(), String>;

pub trait Matcher<T>: fmt::Display {
    fn matches(&self, actual: T) -> MatchResult;
}
