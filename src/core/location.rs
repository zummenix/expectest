
use std::fmt;

/// Represents a location in a source code.
#[derive(Clone, Copy)]
pub struct SourceLocation {
    /// Name of a file.
    pub file: &'static str,
    /// Number of line in the file.
    pub line: u32,
}

impl SourceLocation {
    /// Creates new `SourceLocation` using `file` and `line`.
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
mod tests {
    use super::SourceLocation;

    #[test]
    fn location_string() {
        let location = SourceLocation::new("path/to/file.rs", 9);
        assert!(format!("{}", location) == "path/to/file.rs:9");
    }
}
