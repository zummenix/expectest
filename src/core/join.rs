
use std::fmt;

/// An enum used to join two parts of a failure message.
#[derive(PartialEq, Debug)]
pub enum Join {
    /// "to" join.
    To,
    /// "to not" join.
    ToNot,
    /// "not to" join.
    NotTo,
}

impl fmt::Display for Join {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text())
    }
}

impl Join {
    /// Checks if `Join` is assertion.
    pub fn is_assertion(&self) -> bool {
        *self == Join::To
    }

    fn text(&self) -> &'static str {
        match *self {
            Join::To => "to",
            Join::ToNot => "to not",
            Join::NotTo => "not to",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Join;

    #[test]
    fn join_display_to() {
        assert!(format!("{}", Join::To) == "to");
    }

    #[test]
    fn join_display_to_not() {
        assert!(format!("{}", Join::ToNot) == "to not");
    }

    #[test]
    fn join_display_not_to() {
        assert!(format!("{}", Join::NotTo) == "not to");
    }

    #[test]
    fn join_to_is_assertion() {
        assert!(Join::To.is_assertion());
    }

    #[test]
    fn join_to_not_is_not_assertion() {
        assert!(!Join::ToNot.is_assertion());
    }

    #[test]
    fn join_not_to_is_not_assertion() {
        assert!(!Join::NotTo.is_assertion());
    }
}
