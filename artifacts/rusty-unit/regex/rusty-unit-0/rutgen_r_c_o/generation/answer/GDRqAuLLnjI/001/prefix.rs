// Answer 0

#[test]
fn test_negate_bytes_full_range() {
    let mut class_bytes = Class::Bytes(ClassBytes::new((0..=255).map(|b| ClassBytesRange::new(b, b))));
    class_bytes.negate();
}

#[test]
fn test_negate_bytes_empty() {
    let mut class_bytes = Class::Bytes(ClassBytes::empty());
    class_bytes.negate();
}

#[test]
fn test_negate_bytes_single_value() {
    let mut class_bytes = Class::Bytes(ClassBytes::new(vec![ClassBytesRange::new(100, 100)]));
    class_bytes.negate();
}

#[test]
fn test_negate_bytes_multiple_values() {
    let mut class_bytes = Class::Bytes(ClassBytes::new(vec![ClassBytesRange::new(50, 100), ClassBytesRange::new(200, 200)]));
    class_bytes.negate();
}

#[test]
fn test_negate_bytes_non_contiguous() {
    let mut class_bytes = Class::Bytes(ClassBytes::new(vec![ClassBytesRange::new(0, 10), ClassBytesRange::new(20, 30)]));
    class_bytes.negate();
}

