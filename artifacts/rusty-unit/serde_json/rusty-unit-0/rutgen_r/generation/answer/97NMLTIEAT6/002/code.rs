// Answer 0

#[derive(Debug)]
enum Reference<'a, T> {
    Borrowed(&'a T),
    Copied(T),
}

#[test]
fn test_deref_borrowed() {
    let value = 42;
    let ref_value = Reference::Borrowed(&value);
    
    let result: &i32 = ref_value.deref();
    
    assert_eq!(*result, 42);
}

#[test]
fn test_deref_copied() {
    let value = 42;
    let ref_value = Reference::Copied(value);
    
    let result: i32 = *ref_value.deref();
    
    assert_eq!(result, 42);
}

