//! A module contains matchers.

pub use self::partial_eq::{be_equal_to, BeEqualTo};
pub use self::partial_ord::{be_greater_or_equal_to, be_greater_than, be_less_or_equal_to,
                            be_less_than, PartialOrder};
pub use self::bool::{be_false, be_true, BeFalse, BeTrue};
pub use self::option::{be_none, be_some, BeNone, BeSome};
pub use self::result::{be_err, be_ok, BeErr, BeOk};
pub use self::floats::{be_close_to, BeCloseTo};
pub use self::empty::{be_empty, BeEmpty};
pub use self::count::{have_count, HaveCount};

mod partial_eq;
mod partial_ord;
mod bool;
mod option;
mod result;
mod floats;
mod empty;
mod count;
