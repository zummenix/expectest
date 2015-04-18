
#[macro_export]
macro_rules! expect {
    ($e: expr) => (
        expect($e).location($crate::core::SourceLocation::new(file!(), line!()))
    );
}

pub mod prelude {
    pub use core::expect;
    pub use core::ActualValue;
    pub use core::Matcher;
    pub use matchers::be_equal_to;
}

pub mod core;
pub mod matchers;
