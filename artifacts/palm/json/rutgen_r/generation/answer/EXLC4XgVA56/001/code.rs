// Answer 0

#[test]
fn test_pretty_formatter_new() {
    // Arrange
    let expected_indent = b"  ";

    // Act
    let formatter = serde_json::ser::PrettyFormatter::new();

    // Assert
    assert_eq!(formatter.indent, expected_indent);
}

#[test]
fn test_pretty_formatter_new_edge_case() {
    // Arrange
    // Testing with a different indentation beyond the expected
    let unexpected_indent = b" "; // one space instead of two

    // Act
    let formatter = serde_json::ser::PrettyFormatter::new();

    // Assert
    assert_ne!(formatter.indent, unexpected_indent);
}

