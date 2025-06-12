// Answer 0

#[test]
fn test_with_indent_empty() {
    let indent: &[u8] = b"";
    let formatter = serde_json::with_indent(indent);
    assert_eq!(formatter.current_indent, 0);
    assert_eq!(formatter.has_value, false);
    assert_eq!(formatter.indent, indent);
}

#[test]
fn test_with_indent_single_space() {
    let indent: &[u8] = b" ";
    let formatter = serde_json::with_indent(indent);
    assert_eq!(formatter.current_indent, 0);
    assert_eq!(formatter.has_value, false);
    assert_eq!(formatter.indent, indent);
}

#[test]
fn test_with_indent_tab_character() {
    let indent: &[u8] = b"\t";
    let formatter = serde_json::with_indent(indent);
    assert_eq!(formatter.current_indent, 0);
    assert_eq!(formatter.has_value, false);
    assert_eq!(formatter.indent, indent);
}

#[test]
fn test_with_indent_multiple_spaces() {
    let indent: &[u8] = b"    ";
    let formatter = serde_json::with_indent(indent);
    assert_eq!(formatter.current_indent, 0);
    assert_eq!(formatter.has_value, false);
    assert_eq!(formatter.indent, indent);
}

#[test]
fn test_with_indent_special_characters() {
    let indent: &[u8] = b"\n";
    let formatter = serde_json::with_indent(indent);
    assert_eq!(formatter.current_indent, 0);
    assert_eq!(formatter.has_value, false);
    assert_eq!(formatter.indent, indent);
}

