
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

    #[test]
    fn is_some_value_value() {
        assert!(is_some_value(Some(&5), Some(&5)));
    }

    #[test]
    #[should_panic]
    fn is_some_value_value_should_panic() {
        assert!(is_some_value(Some(&5), Some(&4)));
    }

    #[test]
    fn is_some_value_some() {
        assert!(is_some_value(Some(&5), None));
    }

    #[test]
    #[should_panic]
    fn is_some_value_some_should_panic1() {
        assert!(is_some_value(None::<&u32>, None));
    }

    #[test]
    #[should_panic]
    fn is_some_value_some_should_panic2() {
        assert!(is_some_value(None::<&u32>, Some(&1)));
    }
}
