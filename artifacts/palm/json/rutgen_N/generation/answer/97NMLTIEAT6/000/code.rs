// Answer 0

#[derive(Debug)]
enum Reference<'a, T: 'a> {
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
fn test_deref_borrowed() {
    let value = 42;
    let reference = Reference::Borrowed(&value);
    assert_eq!(*reference.deref(), 42);
}

#[test]
fn test_deref_copied() {
    let value = 84;
    let reference = Reference::Copied(value);
    assert_eq!(*reference.deref(), 84);
}

