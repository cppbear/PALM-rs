// Answer 0

#[test]
fn test_hir_ascii_class_bytes_alnum() {
    let kind = ast::ClassAsciiKind::Alnum;
    let result = hir_ascii_class_bytes(&kind);
    let expected = hir::ClassBytes::new(vec![
        hir::ClassBytesRange::new(b'0', b'9'),
        hir::ClassBytesRange::new(b'A', b'Z'),
        hir::ClassBytesRange::new(b'a', b'z'),
    ]);
    assert_eq!(result, expected);
}

#[test]
fn test_hir_ascii_class_bytes_alpha() {
    let kind = ast::ClassAsciiKind::Alpha;
    let result = hir_ascii_class_bytes(&kind);
    let expected = hir::ClassBytes::new(vec![
        hir::ClassBytesRange::new(b'A', b'Z'),
        hir::ClassBytesRange::new(b'a', b'z'),
    ]);
    assert_eq!(result, expected);
}

#[test]
fn test_hir_ascii_class_bytes_ascii() {
    let kind = ast::ClassAsciiKind::Ascii;
    let result = hir_ascii_class_bytes(&kind);
    let expected = hir::ClassBytes::new(vec![
        hir::ClassBytesRange::new(0, 127),
    ]);
    assert_eq!(result, expected);
}

#[test]
fn test_hir_ascii_class_bytes_blank() {
    let kind = ast::ClassAsciiKind::Blank;
    let result = hir_ascii_class_bytes(&kind);
    let expected = hir::ClassBytes::new(vec![
        hir::ClassBytesRange::new(b' ', b'\t'),
    ]);
    assert_eq!(result, expected);
}

#[test]
fn test_hir_ascii_class_bytes_digit() {
    let kind = ast::ClassAsciiKind::Digit;
    let result = hir_ascii_class_bytes(&kind);
    let expected = hir::ClassBytes::new(vec![
        hir::ClassBytesRange::new(b'0', b'9'),
    ]);
    assert_eq!(result, expected);
}

#[test]
fn test_hir_ascii_class_bytes_graph() {
    let kind = ast::ClassAsciiKind::Graph;
    let result = hir_ascii_class_bytes(&kind);
    let expected = hir::ClassBytes::new(vec![
        hir::ClassBytesRange::new(b'!', b'~'),
    ]);
    assert_eq!(result, expected);
}

#[test]
fn test_hir_ascii_class_bytes_lower() {
    let kind = ast::ClassAsciiKind::Lower;
    let result = hir_ascii_class_bytes(&kind);
    let expected = hir::ClassBytes::new(vec![
        hir::ClassBytesRange::new(b'a', b'z'),
    ]);
    assert_eq!(result, expected);
}

#[test]
fn test_hir_ascii_class_bytes_print() {
    let kind = ast::ClassAsciiKind::Print;
    let result = hir_ascii_class_bytes(&kind);
    let expected = hir::ClassBytes::new(vec![
        hir::ClassBytesRange::new(b' ', b'~'),
    ]);
    assert_eq!(result, expected);
}

#[test]
fn test_hir_ascii_class_bytes_punct() {
    let kind = ast::ClassAsciiKind::Punct;
    let result = hir_ascii_class_bytes(&kind);
    let expected = hir::ClassBytes::new(vec![
        hir::ClassBytesRange::new(b'!', b'/'),
        hir::ClassBytesRange::new(b':', b'@'),
        hir::ClassBytesRange::new(b'[', b'`'),
        hir::ClassBytesRange::new(b'{', b'~'),
    ]);
    assert_eq!(result, expected);
}

#[test]
fn test_hir_ascii_class_bytes_space() {
    let kind = ast::ClassAsciiKind::Space;
    let result = hir_ascii_class_bytes(&kind);
    let expected = hir::ClassBytes::new(vec![
        hir::ClassBytesRange::new(b'\t', b'\t'),
        hir::ClassBytesRange::new(b'\n', b'\n'),
        hir::ClassBytesRange::new(b'\x0B', b'\x0B'),
        hir::ClassBytesRange::new(b'\x0C', b'\x0C'),
        hir::ClassBytesRange::new(b'\r', b'\r'),
        hir::ClassBytesRange::new(b' ', b' '),
    ]);
    assert_eq!(result, expected);
}

#[test]
fn test_hir_ascii_class_bytes_upper() {
    let kind = ast::ClassAsciiKind::Upper;
    let result = hir_ascii_class_bytes(&kind);
    let expected = hir::ClassBytes::new(vec![
        hir::ClassBytesRange::new(b'A', b'Z'),
    ]);
    assert_eq!(result, expected);
}

#[test]
fn test_hir_ascii_class_bytes_word() {
    let kind = ast::ClassAsciiKind::Word;
    let result = hir_ascii_class_bytes(&kind);
    let expected = hir::ClassBytes::new(vec![
        hir::ClassBytesRange::new(b'0', b'9'),
        hir::ClassBytesRange::new(b'A', b'Z'),
        hir::ClassBytesRange::new(b'_', b'_'),
        hir::ClassBytesRange::new(b'a', b'z'),
    ]);
    assert_eq!(result, expected);
}

#[test]
fn test_hir_ascii_class_bytes_xdigit() {
    let kind = ast::ClassAsciiKind::Xdigit;
    let result = hir_ascii_class_bytes(&kind);
    let expected = hir::ClassBytes::new(vec![
        hir::ClassBytesRange::new(b'0', b'9'),
        hir::ClassBytesRange::new(b'A', b'F'),
        hir::ClassBytesRange::new(b'a', b'f'),
    ]);
    assert_eq!(result, expected);
}

