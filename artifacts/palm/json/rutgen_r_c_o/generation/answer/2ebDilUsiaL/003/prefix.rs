// Answer 0

#[test]
fn test_indent_zero_iterations() {
    let mut buffer = Vec::new();
    let s = b"";
    let n = 0;
    let result = indent(&mut buffer, n, s);
}

#[test]
fn test_indent_empty_string() {
    let mut buffer = Vec::new();
    let s = b"";
    let n = 0;
    let result = indent(&mut buffer, n, s);
}

