
#[macro_export]
macro_rules! expect {
    ($e: expr) => (
        expect($e).location($crate::core::SourceLocation::new(file!(), line!()))
    );
}

pub mod prelude {
    pub use core::{ expect, ActualValue, Matcher };
    pub use matchers::{
        be_equal_to,
        be_true,
        be_false,
    };
}

pub mod core;
pub mod matchers;
