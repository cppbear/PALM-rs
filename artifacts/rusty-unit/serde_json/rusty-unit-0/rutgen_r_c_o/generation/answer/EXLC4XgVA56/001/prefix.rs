// Answer 0

#[test]
fn test_pretty_formatter_new_default() {
    let formatter = PrettyFormatter::new();
}

#[test]
fn test_pretty_formatter_with_indent_space() {
    let formatter = PrettyFormatter::with_indent(b" ");
}

#[test]
fn test_pretty_formatter_with_indent_tab() {
    let formatter = PrettyFormatter::with_indent(b"\t");
}

#[test]
fn test_pretty_formatter_with_indent_spaces() {
    let formatter = PrettyFormatter::with_indent(b"    ");
}

#[test]
fn test_pretty_formatter_with_indent_custom() {
    let formatter = PrettyFormatter::with_indent(b"  \t");
}

#[test]
fn test_pretty_formatter_with_indent_mixed() {
    let formatter = PrettyFormatter::with_indent(b" a b ");
}

#[test]
fn test_pretty_formatter_with_empty_indent() {
    let formatter = PrettyFormatter::with_indent(b"");
}

#[test]
fn test_pretty_formatter_with_indent_exceeds_limit() {
    let formatter = PrettyFormatter::with_indent(b"1234567890"); // Exactly 10 characters
}

#[test]
#[should_panic]
fn test_pretty_formatter_with_invalid_indent() {
    let formatter = PrettyFormatter::with_indent(b"12345678901"); // Exceeds 10 characters
}

