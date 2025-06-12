// Answer 0

#[test]
fn test_visit_borrowed_bytes_empty() {
    let visitor = ContentVisitor { value: PhantomData };
    let value: &[u8] = &[];
    let result = visitor.visit_borrowed_bytes(value);
    assert_eq!(result.unwrap(), Content::Bytes(value.to_vec()));
}

#[test]
fn test_visit_borrowed_bytes_single_byte() {
    let visitor = ContentVisitor { value: PhantomData };
    let value: &[u8] = &[1];
    let result = visitor.visit_borrowed_bytes(value);
    assert_eq!(result.unwrap(), Content::Bytes(value.to_vec()));
}

#[test]
fn test_visit_borrowed_bytes_multiple_bytes() {
    let visitor = ContentVisitor { value: PhantomData };
    let value: &[u8] = &[1, 2, 3, 4, 5];
    let result = visitor.visit_borrowed_bytes(value);
    assert_eq!(result.unwrap(), Content::Bytes(value.to_vec()));
}

#[test]
fn test_visit_borrowed_bytes_large_input() {
    let visitor = ContentVisitor { value: PhantomData };
    let value: &[u8] = &[0; 1024]; // large input of 1024 bytes
    let result = visitor.visit_borrowed_bytes(value);
    assert_eq!(result.unwrap(), Content::Bytes(value.to_vec()));
}

#[test]
#[should_panic]
fn test_visit_borrowed_bytes_null() {
    let visitor = ContentVisitor { value: PhantomData };
    let value: &[u8] = std::ptr::null(); // this will panic if dereferenced
    let _result = visitor.visit_borrowed_bytes(value);
}

