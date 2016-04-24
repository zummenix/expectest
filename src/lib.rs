
//! Crate provides matchers and matcher functions for unit testing.

extern crate num_traits;

/// A macro intended to use instead of `expect` function.
///
/// Provides a file name and a line number for a failed test case.
#[macro_export]
macro_rules! expect {
    ($e: expr) => (
        $crate::core::expect($e).location($crate::core::SourceLocation::new(file!(), line!()))
    );
}

pub mod prelude {
    //! A module contains reexport of all useful functions.

    pub use core::expect;
    pub use matchers::{be_equal_to, be_less_than, be_less_or_equal_to, be_greater_than,
                       be_greater_or_equal_to, be_true, be_false, be_some, be_none, be_ok, be_err,
                       be_close_to, be_empty, have_count};
}

pub mod core;
pub mod matchers;
