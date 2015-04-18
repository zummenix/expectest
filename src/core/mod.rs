
use std::fmt;

pub use self::location::SourceLocation;
pub use self::actual::{ expect, ActualValue, failure };

mod location;
mod actual;

pub type MatchResult = Result<(), String>;

pub trait Matcher<T>: fmt::Display {
    fn matches(&self, actual: T) -> MatchResult;
}
