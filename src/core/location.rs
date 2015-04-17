
use std::fmt;

#[derive(Clone, Copy)]
pub struct SourceLocation {
    pub file: &'static str,
    pub line: u32,
}

impl SourceLocation {
    pub fn new(file: &'static str, line: u32) -> SourceLocation {
        SourceLocation {
            file: file,
            line: line,
        }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.file, self.line)
    }
}

#[cfg(test)]
mod test {
    use super::SourceLocation;

    #[test]
    fn location_string() {
        let location = SourceLocation::new("path/to/file.rs", 9);
        assert!(format!("{}", location) == "path/to/file.rs:9");
    }
}
