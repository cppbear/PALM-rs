// Answer 0

#[test]
fn test_any_with_bytes_true() {
    let result = any(true);
}

#[test]
fn test_any_with_bytes_true_using_lit() {
    let char_range = ClassBytesRange::new(b'A', b'Z');
    let mut class_bytes = ClassBytes::empty();
    class_bytes.push(char_range);
    let result = Hir::class(Class::Bytes(class_bytes));
}

#[test]
fn test_any_with_bytes_true_empty_range() {
    let char_range = ClassBytesRange::new(b'\0', b'\0');
    let mut class_bytes = ClassBytes::empty();
    class_bytes.push(char_range);
    let result = Hir::class(Class::Bytes(class_bytes));
}

#[test]
fn test_any_with_bytes_true_full_range() {
    let char_range = ClassBytesRange::new(b'\0', b'\xFF');
    let mut class_bytes = ClassBytes::empty();
    class_bytes.push(char_range);
    let result = Hir::class(Class::Bytes(class_bytes));
}

