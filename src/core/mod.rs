
//! A module contains core types of this library.

pub use self::location::SourceLocation;
pub use self::actual::{expect, ActualValue};
pub use self::join::Join;
pub use self::failure::{Failure, panics_with_message};

mod location;
mod actual;
mod join;
mod failure;

/// A Matcher trait.
pub trait Matcher<A, E> {
    /// Returns a failure message.
    fn failure_message(&self, join: Join, actual: &A) -> String;

    /// Checks if an actual value matches an expected value.
    fn matches(&self, actual: &A) -> bool;
}
