
use std::io;
use core::SourceLocation;

#[derive(Debug)]
pub enum TestResult {
    Success,
    Failure(Failure),
}

impl TestResult {
    pub fn new_failure(message: String, location: Option<SourceLocation>) -> Self {
        TestResult::Failure(Failure { should_panic: true, message: message, location: location })
    }
}

#[derive(Debug)]
pub struct Failure {
    should_panic: bool,
    message: String,
    location: Option<SourceLocation>,
}

impl Drop for TestResult {
    fn drop(&mut self) {
        if let TestResult::Failure(ref f) = *self {
            if f.should_panic {
                let mut text = "\n".to_owned();
                if let Some(l) = f.location {
                    text.push_str(&l.to_string());
                    text.push_str("\n");
                }
                text.push_str(&f.message);
                text.push_str("\n\n");
                io::copy(&mut text.as_bytes(), &mut io::stdout()).unwrap();
                panic!("test failure");
            }
        }
    }
}

pub fn panics_with_message<F>(message: &str, f: F) where F: Fn() -> TestResult {
    if let TestResult::Failure(ref mut failure) = f() {
        failure.should_panic = false;
        assert_eq!(failure.message, message);
    } else {
        panic!("");
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
