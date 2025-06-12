// Answer 0

#[test]
fn test_hir_ascii_class_bytes_lowercase() {
    use regex_syntax::ast;
    use regex_syntax::hir;

    // Create an instance of ClassAsciiKind for lowercase letters
    let kind = ast::ClassAsciiKind::Lowercase;

    // Call the function under test
    let class_bytes = hir_ascii_class_bytes(&kind);

    // Validate the produced ClassBytes structure
    assert_eq!(class_bytes.ranges.len(), 1);
    assert_eq!(class_bytes.ranges[0].start, b'a');
    assert_eq!(class_bytes.ranges[0].end, b'z');
}

#[test]
fn test_hir_ascii_class_bytes_uppercase() {
    use regex_syntax::ast;
    use regex_syntax::hir;

    // Create an instance of ClassAsciiKind for uppercase letters
    let kind = ast::ClassAsciiKind::Uppercase;

    // Call the function under test
    let class_bytes = hir_ascii_class_bytes(&kind);

    // Validate the produced ClassBytes structure
    assert_eq!(class_bytes.ranges.len(), 1);
    assert_eq!(class_bytes.ranges[0].start, b'A');
    assert_eq!(class_bytes.ranges[0].end, b'Z');
}

#[test]
fn test_hir_ascii_class_bytes_digits() {
    use regex_syntax::ast;
    use regex_syntax::hir;

    // Create an instance of ClassAsciiKind for digits
    let kind = ast::ClassAsciiKind::Digits;

    // Call the function under test
    let class_bytes = hir_ascii_class_bytes(&kind);

    // Validate the produced ClassBytes structure
    assert_eq!(class_bytes.ranges.len(), 1);
    assert_eq!(class_bytes.ranges[0].start, b'0');
    assert_eq!(class_bytes.ranges[0].end, b'9');
}

