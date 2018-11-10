//! A module contains matchers.

pub use self::bool::{be_false, be_true, BeFalse, BeTrue};
pub use self::count::{have_count, HaveCount};
pub use self::empty::{be_empty, BeEmpty};
pub use self::floats::{be_close_to, BeCloseTo};
pub use self::option::{be_none, be_some, BeNone, BeSome};
pub use self::partial_eq::{be_equal_to, be_equal_to as be_eq, BeEqualTo};
pub use self::partial_ord::{be_greater_or_equal_to, be_greater_or_equal_to as be_ge,
                            be_greater_than, be_greater_than as be_gt, be_less_or_equal_to,
                            be_less_or_equal_to as be_le, be_less_than, be_less_than as be_lt,
                            PartialOrder};
pub use self::range::{be_within_range, BeWithinRange};
pub use self::result::{be_err, be_ok, BeErr, BeOk};

mod bool;
mod count;
mod empty;
mod floats;
mod option;
mod partial_eq;
mod partial_ord;
mod range;
mod result;
