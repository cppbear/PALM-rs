// Answer 0

#[test]
fn test_new_pretty_formatter() {
    use serde_json::ser::PrettyFormatter;

    // Test that the PrettyFormatter is constructed with two spaces for indentation
    let formatter = PrettyFormatter::new();
    let expected_indent = b"  ";
    
    assert_eq!(formatter.indent, expected_indent);
}

