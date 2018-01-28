use core::SourceLocation;

/// Represents result of unit testing.
#[derive(Debug)]
pub enum TestResult {
    /// Denotes success.
    Success,
    /// Denotes failure.
    Failure(Failure),
}

impl TestResult {
    /// Creates a new instance of `TestResult` with a `Success` variant.
    pub fn new_success() -> Self {
        TestResult::Success
    }

    /// Creates a new instance of `TestResult` with a `Failure` variant using message
    /// and location.
    pub fn new_failure(message: String, location: Option<SourceLocation>) -> Self {
        TestResult::Failure(Failure {
            should_panic: true,
            message: message,
            location: location,
        })
    }

    /// Asserts that `message` is equal to the failure message.
    ///
    /// Purpose of this method is the unit testing of failure messages. Convenient for
    /// development.
    ///
    /// # Example
    /// ```
    /// # use expectest::prelude::*;
    /// expect(0 == 1).to(be_true()).assert_eq_message("expected to be true");
    /// ```
    ///
    /// # Panics
    /// This method panics if the failure message is not equal to the `message`, or `self` is not
    /// the `TestResult::Failure` variant.
    pub fn assert_eq_message(self, message: &str) {
        if let TestResult::Failure(mut failure) = self {
            failure.should_panic = false;
            assert_eq!(failure.message, message);
        } else {
            panic!("expected to be TestResult::Failure, got <{:?}>", self);
        }
    }
}

/// Represents data for `Failure` variant of a `TestResult`.
///
/// There is no public constructor for this struct. To create a new instance you should
/// use `TestResult::new_failure` method.
///
/// This struct implements the `Drop` trait to print failure message and panic.
#[derive(Debug)]
pub struct Failure {
    should_panic: bool,
    message: String,
    location: Option<SourceLocation>,
}

impl Failure {
    #[cfg(feature = "nightly")]
    fn panic(&self) {
        use rust_core::panicking;

        if let Some(location) = self.location {
            let file_line = &(location.file, location.line, location.column);
            panicking::panic_fmt(format_args!("{}", self.message), file_line);
        } else {
            panic!("assertion failed: `{}`", self.message);
        }
    }

    #[cfg(not(feature = "nightly"))]
    fn panic(&self) {
        if let Some(location) = self.location {
            panic!("assertion failed: `{}`, {}", self.message, location);
        } else {
            panic!("assertion failed: `{}`", self.message);
        }
    }
}

impl Drop for Failure {
    fn drop(&mut self) {
        if self.should_panic {
            self.panic();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TestResult;

    #[test]
    #[should_panic]
    fn it_panics() {
        TestResult::new_failure("panics on drop".into(), None);
    }
}
