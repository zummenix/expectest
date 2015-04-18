
pub use self::location::SourceLocation;
pub use self::actual::{ expect, ActualValue, failure };

mod location;
mod actual;

pub trait Matcher<A, E> {
    fn format_message(&self, join: &'static str, actual: &A) -> String;
    fn matches(&self, actual: &A) -> bool;
}
