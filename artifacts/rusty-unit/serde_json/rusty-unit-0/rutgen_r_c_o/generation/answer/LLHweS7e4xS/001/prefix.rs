// Answer 0

#[test]
fn test_with_indent_empty_array() {
    let indent: &[u8] = &[];
    let formatter = PrettyFormatter::with_indent(indent);
}

#[test]
fn test_with_indent_single_element() {
    let indent: &[u8] = &[b' '];
    let formatter = PrettyFormatter::with_indent(indent);
}

#[test]
fn test_with_indent_max_length() {
    let indent: &[u8] = &[b'a'; 255];
    let formatter = PrettyFormatter::with_indent(indent);
}

#[test]
#[should_panic]
fn test_with_indent_exceeding_length() {
    let indent: &[u8] = &[b'a'; 256];
    let formatter = PrettyFormatter::with_indent(indent);
}

