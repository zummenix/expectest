
/// An IsEmpty trait for types that can be checked for emptiness.
pub trait IsEmpty {

    /// Checks if something is empty.
    fn is_empty(&self) -> bool;
}

impl IsEmpty for String {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<'a> IsEmpty for &'a str {
    fn is_empty(&self) -> bool {
        (*self).is_empty()
    }
}

impl<T> IsEmpty for Vec<T> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<'a, T> IsEmpty for &'a [T] {
    fn is_empty(&self) -> bool {
        (*self).is_empty()
    }
}
