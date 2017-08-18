
use std::fmt;

/// Represents a location in a source code.
#[derive(Clone, Copy)]
pub struct SourceLocation {
    /// Name of a file.
    pub file: &'static str,
    /// Number of line in the file.
    pub line: u32,
    /// Number of column in the line.
    pub column: u32,
}

impl SourceLocation {
    /// Creates a new instance of `SourceLocation`.
    pub fn new(file: &'static str, line: u32, column: u32) -> SourceLocation {
        SourceLocation { file, line, column }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

impl fmt::Debug for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::SourceLocation;
    use matchers::be_equal_to;

    #[test]
    fn location_should_display_correct_text() {
        let location = SourceLocation::new("path/to/file.rs", 9, 8);
        expect!(location.to_string()).to(be_equal_to("path/to/file.rs:9:8"));
    }
}
