
pub use self::location::SourceLocation;
pub use self::actual::{ expect, ActualValue, failure };

mod location;
mod actual;

/// A Matcher trait.
pub trait Matcher<A, E> {

    /// Returns a failure message.
    ///
    /// `join` - The string that should be used to format a proper message.
    /// Currently library supports: "to", "to not" and "not to".
    ///
    /// `actual` - The actual value.
    fn failure_message(&self, join: &'static str, actual: &A) -> String;

    /// Returns `true` if an actual value matches an expected value.
    ///
    /// `actual` - The actual value.
    fn matches(&self, actual: &A) -> bool;
}
