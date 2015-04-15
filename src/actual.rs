
use SourceLocation;
use Matcher;

pub fn expect<T>(value: T, location: SourceLocation) -> ActualValue<T> {
    ActualValue::new(value, location)
}

pub struct ActualValue<T> {
    value: T,
    location: SourceLocation,
}

impl<T> ActualValue<T> {
    fn new(value: T, location: SourceLocation) -> ActualValue<T> {
        ActualValue {
            value: value,
            location: location,
        }
    }

    pub fn to<M>(self, matcher: M) where M: Matcher<T> {
        let location = self.location;
        if let Err(mismatch) = matcher.matches(self.value) {
            failure(format!("\nexpected <{}>, got <{}>", matcher, mismatch), location);
        }
    }
}

pub fn failure(message: String, location: SourceLocation) {
    println!("{}", message);
    ::std::rt::unwind::begin_unwind("tests", &(location.file, location.line as usize));
}

#[cfg(test)]
mod test {
    use super::*;
    use SourceLocation;

    #[test]
    fn new_expectation() {
        expect([1, 2, 3], SourceLocation::new(file!(), line!()));
    }
}
