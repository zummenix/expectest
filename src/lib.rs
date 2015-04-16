
pub use location::SourceLocation;
pub use actual::{ expect, ActualValue };
pub use matcher::{ Matcher, MatchResult };
pub use matchers::equal::{ equal };

#[macro_export]
macro_rules! expect {
    ($e: expr) => (
        expect($e).location($crate::SourceLocation::new(file!(), line!()))
    );
}

pub mod prelude {
    pub use expect;
    pub use ActualValue;
    pub use Matcher;

    pub use equal;
}


pub mod location;
pub mod actual;
pub mod matcher;
pub mod matchers {
    pub mod equal;
}
