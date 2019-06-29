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
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let join = match *self {
            Join::To => "to",
            Join::ToNot => "to not",
            Join::NotTo => "not to",
        };
        fmt.write_str(join)
    }
}

impl Join {
    /// Checks if `Join` is assertion.
    pub fn is_assertion(&self) -> bool {
        *self == Join::To
    }
}

#[cfg(test)]
mod tests {
    use super::Join;
    use crate::matchers::{be_equal_to, be_false, be_true};

    #[test]
    fn join_should_display_correct_text() {
        expect!(Join::To.to_string()).to(be_equal_to("to"));
        expect!(Join::ToNot.to_string()).to(be_equal_to("to not"));
        expect!(Join::NotTo.to_string()).to(be_equal_to("not to"));
    }

    #[test]
    fn join_might_be_assertion_or_negation() {
        expect!(Join::To.is_assertion()).to(be_true());
        expect!(Join::ToNot.is_assertion()).to(be_false());
        expect!(Join::NotTo.is_assertion()).to(be_false());
    }
}
