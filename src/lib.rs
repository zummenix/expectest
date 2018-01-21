//! This project is inspired by [Nimble](https://github.com/Quick/Nimble) for Swift.
//! It provides matchers and matcher functions to express expectations in tests for common cases,
//! such as: Equality, Order, Option, Result, etc.
//! The crate also provides instruments to implement custom matchers for your project's domain.
//!
//! You express expectations in your tests using three basic constructs:
//!
//! ```rust,ignore
//! expect!(...).to(...);
//! expect!(...).to_not(...);
//! expect!(...).not_to(...);
//! ```
//!
//! If a test fails, you will see the reason in a nice, human readable format.
//!
//! # Example
//!
//! ```rust,should_panic
//! # #[macro_use] extern crate expectest;
//! # use expectest::prelude::*;
//! # fn main() {
//! let result = vec![1, 2, 2];
//! expect!(result).to(be_equal_to([1, 2, 3]));
//! # }
//! // --
//! // This will print:
//! // expected to be equal to <[1, 2, 3]>, got <[1, 2, 2]>
//! ```
//!
//! # Structure
//!
//! The crate consists of two modules `core` and `matchers` but for general use
//! you can access all you need using `prelude` module. The `expect!` macro saves
//! file name and line number to print it later if an expectation fails. Internally the macro uses
//! the `expect` function which is also available to you.
//!
//! # Usage
//!
//! The crate is meant to be used in tests, so we recommend you include it as a dev dependency
//! with `#[cfg(test)]` attribute.
//!
//! In your Cargo.toml:
//!
//! ```toml
//! [dev-dependencies]
//! expectest = "0.9.2"
//! ```
//!
//! If you prefer nightly rust and want failure messages to be integrated in rust's
//! standard panic message, enable `nightly` feature:
//!
//! ```toml
//! [dev-dependencies]
//! expectest = { version = "0.9.2", features = ["nightly"] }
//! ```
//!
//! In your crate root:
//!
//! ```rust,ignore
//! #[cfg(test)]
//! #[macro_use(expect)]
//! extern crate expectest;
//! ```
//!
//! In your tests:
//!
//! ```rust,ignore
//! use expectest::prelude::*;
//! ```
//! # Matchers
//!
//! Keep in mind that some matchers work with types that implement `Debug` trait
//! to print inner representation. You need to derive it for your types.
//!
//! ## Equality
//!
//! For types that implement `PartialEq` trait.
//!
//! ```rust
//! # #[macro_use] extern crate expectest;
//! # use expectest::prelude::*;
//! # fn main() {
//! expect!("hello".to_string()).to(be_equal_to("hello"));
//! # }
//! ```
//!
//! ## Equality of Floats
//!
//! With default `delta` equal to `0.001`.
//!
//! ```rust
//! # #[macro_use] extern crate expectest;
//! # use expectest::prelude::*;
//! # fn main() {
//! expect!(12.001_f64).to(be_close_to(12.0));
//! expect!(12.1_f64).to(be_close_to(12.0).delta(0.1));
//! # }
//! ```
//!
//! ## Order
//!
//! For types that implement `PartialOrd` trait.
//!
//! Use any of the following matchers:
//!
//! - `be_less_than`
//! - `be_less_or_equal_to`
//! - `be_greater_than`
//! - `be_greater_or_equal_to`
//!
//! ```rust
//! # #[macro_use] extern crate expectest;
//! # use expectest::prelude::*;
//! # fn main() {
//! expect!(1).to(be_greater_than(0));
//! # }
//! ```
//!
//! ## Option\<T\>
//!
//! Use `be_some` or `be_none` matchers.
//!
//! ```rust
//! # #[macro_use] extern crate expectest;
//! # use expectest::prelude::*;
//! # fn main() {
//! expect!(Some(9)).to(be_some());
//! expect!(Some(9)).to(be_some().value(9));
//! # }
//! ```
//!
//! ## Result\<T, E\>
//!
//! Use `be_ok` or `be_err` matchers.
//!
//! ```rust
//! # #[macro_use] extern crate expectest;
//! # use expectest::prelude::*;
//! # fn main() {
//! expect!("4".parse::<u32>()).to(be_ok());
//! expect!("4".parse::<u32>()).to(be_ok().value(4));
//! # }
//! ```
//!
//! ## Iterators
//!
//! For types that implement `Iterator + Clone` trait.
//!
//! Use `be_empty` or `have_count` matchers.
//!
//! ```rust
//! # #[macro_use] extern crate expectest;
//! # use expectest::prelude::*;
//! # fn main() {
//! expect!("".chars()).to(be_empty());
//! expect!("abc".chars()).to(have_count(3));
//! # }
//! ```
//!
//! ## Boolean
//!
//! Use `be_true` or `be_false` matchers.
//!
//! ```rust
//! # #[macro_use] extern crate expectest;
//! # use expectest::prelude::*;
//! # fn main() {
//! expect!(9 == 9).to(be_true());
//! # }
//! ```
//!
//! That's all you need to know.
//!
//! **Happy Testing 😊**
//!

#![cfg_attr(feature = "nightly", feature(core_panic))]

extern crate num_traits;

#[cfg(feature = "nightly")]
extern crate core as rust_core;

/// A macro intended to use instead of `expect` function.
///
/// Provides a file name and a line number for a failed test case.
#[macro_export]
macro_rules! expect {
    ($e: expr) => ({
        let location = $crate::core::SourceLocation::new(file!(), line!(), column!());
        $crate::core::expect($e).location(location)
    });
}

pub mod prelude {
    //! A module contains reexport of all useful functions.

    pub use core::expect;
    pub use matchers::{be_close_to, be_empty, be_equal_to, be_err, be_false,
                       be_greater_or_equal_to, be_greater_than, be_less_or_equal_to, be_less_than,
                       be_none, be_ok, be_some, be_true, have_count};
}

pub mod core;
pub mod matchers;
