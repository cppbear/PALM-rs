// Answer 0

#[test]
fn test_empty_class_bytes() {
    let class_bytes = regex_syntax::hir::ClassBytes::empty();
    assert_eq!(class_bytes.ranges.len(), 0);
}

#[test]
fn test_empty_class_bytes_not_null() {
    let class_bytes = regex_syntax::hir::ClassBytes::empty();
    assert!(class_bytes.ranges.is_empty());
}

