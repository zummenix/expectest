
//! A crate provides matchers for unit testing.

extern crate num;

/// A macro used to simplify usage of this library.
///
/// You can also use `expect` function instead.
#[macro_export]
macro_rules! expect {
    ($e: expr) => (
        expect($e).location($crate::core::SourceLocation::new(file!(), line!()))
    );
}

pub mod prelude {
    //! A module contains reexport of all useful types and functions.

    pub use core::{ expect, ActualValue, Matcher };
    pub use matchers::{
        be_equal_to,
        be_less_than, be_less_or_equal_to,
        be_greater_than, be_greater_or_equal_to,
        be_true, be_false,
        be_some, BeSome, be_none,
        be_close_to, BeCloseTo,
        be_empty,
    };
}

pub mod core;
pub mod matchers;
pub mod traits;
