
//! An utils module.

/// Checks equality of inner values of `some` and `value`. If `value` is
/// `Option::None` checks that `some` is `Option::Some`.
pub fn is_some_value<T, U>(some: Option<&T>, value: Option<&U>) -> bool
    where T: PartialEq<U>
{
    match (some, value) {
        (Some(s), Some(v)) => s == v,
        (Some(_), None) => true,
        (_, _) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::expect;
    use matchers::{be_true, be_false};

    #[test]
    fn test_is_some_value() {
        expect!(is_some_value(Some(&5), Some(&5))).to(be_true());
        expect!(is_some_value(Some(&5), Some(&4))).to(be_false());
        expect!(is_some_value(Some(&5), None)).to(be_true());
        expect!(is_some_value(None::<&u32>, None)).to(be_false());
        expect!(is_some_value(None::<&u32>, Some(&1))).to(be_false());
    }
}
