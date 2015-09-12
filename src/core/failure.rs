
use std::io;
use core::SourceLocation;

#[derive(Debug)]
pub struct Failure {
    should_panic: bool,
    message: String,
    location: Option<SourceLocation>,
}

impl Failure {
    pub fn new(message: String, location: Option<SourceLocation>) -> Self {
        Failure { should_panic: true, message: message, location: location }
    }

    fn full_message(&self) -> String {
        let mut text = "\n".to_owned();
        if let Some(l) = self.location {
            text.push_str(&l.to_string());
            text.push_str("\n");
        }
        text.push_str(&self.message);
        text.push_str("\n\n");
        text
    }
}

impl Drop for Failure {
    fn drop(&mut self) {
        if self.should_panic {
            io::copy(&mut self.full_message().as_bytes(), &mut io::stdout()).unwrap();
            panic!("test failure");
        }
    }
}

pub fn panics_with_message<F>(message: &str, f: F) where F: Fn() -> Option<Failure> {
    if let Some(mut failure) = f() {
        failure.should_panic = false;
        assert_eq!(failure.message, message);
    } else {
        panic!("");
    }
}

#[cfg(test)]
mod tests {
    use super::Failure;

    #[test]
    #[should_panic]
    fn it_panics() {
        Failure::new("panics on drop".into(), None);
    }
}
