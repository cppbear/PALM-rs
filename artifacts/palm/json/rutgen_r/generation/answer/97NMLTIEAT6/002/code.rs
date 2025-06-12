// Answer 0

#[derive(Debug)]
enum Reference<'a, T: 'a> {
    Borrowed(&'a T),
    Copied(T),
}

#[test]
fn test_deref_borrowed() {
    let value = 42;
    let reference = Reference::Borrowed(&value);
    let result = deref(&reference);
    assert_eq!(*result, 42);
}

#[test]
fn test_deref_copied() {
    let value = 42;
    let reference = Reference::Copied(value);
    let result = deref(&reference);
    assert_eq!(*result, 42);
}

fn deref<T>(reference: &Reference<T>) -> &T {
    match *reference {
        Reference::Borrowed(b) => b,
        Reference::Copied(ref c) => c,
    }
}

