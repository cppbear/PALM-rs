// Answer 0

#[derive(Debug)]
enum Reference<'a, T> {
    Borrowed(&'a T),
    Copied(T),
}

impl<'a, T> Reference<'a, T> {
    fn deref(&self) -> &T {
        match *self {
            Reference::Borrowed(b) => b,
            Reference::Copied(ref c) => c,
        }
    }
}

#[test]
fn test_deref_copied() {
    let value = 42;
    let reference = Reference::Copied(value);
    let result = reference.deref();
    assert_eq!(*result, 42);
}

#[test]
fn test_deref_borrowed() {
    let value = 42;
    let borrowed_value = &value;
    let reference = Reference::Borrowed(borrowed_value);
    let result = reference.deref();
    assert_eq!(*result, 42);
}

