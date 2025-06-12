// Answer 0

#[test]
fn test_hir_ascii_class_bytes_alnum() {
    let kind = &ast::ClassAsciiKind::Alnum;
    let result = hir_ascii_class_bytes(kind);
    let expected_ranges = vec![
        hir::ClassBytesRange::new('0' as u8, '9' as u8),
        hir::ClassBytesRange::new('A' as u8, 'Z' as u8),
        hir::ClassBytesRange::new('a' as u8, 'z' as u8),
    ];
    assert_eq!(result.set, hir::ClassBytes::new(expected_ranges).set);
}

#[test]
fn test_hir_ascii_class_bytes_alpha() {
    let kind = &ast::ClassAsciiKind::Alpha;
    let result = hir_ascii_class_bytes(kind);
    let expected_ranges = vec![
        hir::ClassBytesRange::new('A' as u8, 'Z' as u8),
        hir::ClassBytesRange::new('a' as u8, 'z' as u8),
    ];
    assert_eq!(result.set, hir::ClassBytes::new(expected_ranges).set);
}

#[test]
fn test_hir_ascii_class_bytes_ascii() {
    let kind = &ast::ClassAsciiKind::Ascii;
    let result = hir_ascii_class_bytes(kind);
    let expected_ranges = vec![
        hir::ClassBytesRange::new('\x00' as u8, '\x7F' as u8),
    ];
    assert_eq!(result.set, hir::ClassBytes::new(expected_ranges).set);
}

#[test]
fn test_hir_ascii_class_bytes_blank() {
    let kind = &ast::ClassAsciiKind::Blank;
    let result = hir_ascii_class_bytes(kind);
    let expected_ranges = vec![
        hir::ClassBytesRange::new(' ' as u8, '\t' as u8),
    ];
    assert_eq!(result.set, hir::ClassBytes::new(expected_ranges).set);
}

#[test]
fn test_hir_ascii_class_bytes_digit() {
    let kind = &ast::ClassAsciiKind::Digit;
    let result = hir_ascii_class_bytes(kind);
    let expected_ranges = vec![
        hir::ClassBytesRange::new('0' as u8, '9' as u8),
    ];
    assert_eq!(result.set, hir::ClassBytes::new(expected_ranges).set);
}

#[test]
fn test_hir_ascii_class_bytes_graph() {
    let kind = &ast::ClassAsciiKind::Graph;
    let result = hir_ascii_class_bytes(kind);
    let expected_ranges = vec![
        hir::ClassBytesRange::new('!' as u8, '~' as u8),
    ];
    assert_eq!(result.set, hir::ClassBytes::new(expected_ranges).set);
}

#[test]
fn test_hir_ascii_class_bytes_lower() {
    let kind = &ast::ClassAsciiKind::Lower;
    let result = hir_ascii_class_bytes(kind);
    let expected_ranges = vec![
        hir::ClassBytesRange::new('a' as u8, 'z' as u8),
    ];
    assert_eq!(result.set, hir::ClassBytes::new(expected_ranges).set);
}

#[test]
fn test_hir_ascii_class_bytes_print() {
    let kind = &ast::ClassAsciiKind::Print;
    let result = hir_ascii_class_bytes(kind);
    let expected_ranges = vec![
        hir::ClassBytesRange::new(' ' as u8, '~' as u8),
    ];
    assert_eq!(result.set, hir::ClassBytes::new(expected_ranges).set);
}

#[test]
fn test_hir_ascii_class_bytes_punct() {
    let kind = &ast::ClassAsciiKind::Punct;
    let result = hir_ascii_class_bytes(kind);
    let expected_ranges = vec![
        hir::ClassBytesRange::new('!' as u8, '/') as hir::ClassBytesRange,
        hir::ClassBytesRange::new(':' as u8, '@' as u8),
        hir::ClassBytesRange::new('[' as u8, '`' as u8),
        hir::ClassBytesRange::new('{' as u8, '~' as u8),
    ];
    assert_eq!(result.set, hir::ClassBytes::new(expected_ranges).set);
}

#[test]
fn test_hir_ascii_class_bytes_space() {
    let kind = &ast::ClassAsciiKind::Space;
    let result = hir_ascii_class_bytes(kind);
    let expected_ranges = vec![
        hir::ClassBytesRange::new('\t' as u8, '\t' as u8),
        hir::ClassBytesRange::new('\n' as u8, '\n' as u8),
        hir::ClassBytesRange::new('\x0B' as u8, '\x0B' as u8),
        hir::ClassBytesRange::new('\x0C' as u8, '\x0C' as u8),
        hir::ClassBytesRange::new('\r' as u8, '\r' as u8),
        hir::ClassBytesRange::new(' ' as u8, ' ' as u8),
    ];
    assert_eq!(result.set, hir::ClassBytes::new(expected_ranges).set);
}

#[test]
fn test_hir_ascii_class_bytes_upper() {
    let kind = &ast::ClassAsciiKind::Upper;
    let result = hir_ascii_class_bytes(kind);
    let expected_ranges = vec![
        hir::ClassBytesRange::new('A' as u8, 'Z' as u8),
    ];
    assert_eq!(result.set, hir::ClassBytes::new(expected_ranges).set);
}

#[test]
fn test_hir_ascii_class_bytes_word() {
    let kind = &ast::ClassAsciiKind::Word;
    let result = hir_ascii_class_bytes(kind);
    let expected_ranges = vec![
        hir::ClassBytesRange::new('0' as u8, '9' as u8),
        hir::ClassBytesRange::new('A' as u8, 'Z' as u8),
        hir::ClassBytesRange::new('_' as u8, '_' as u8),
        hir::ClassBytesRange::new('a' as u8, 'z' as u8),
    ];
    assert_eq!(result.set, hir::ClassBytes::new(expected_ranges).set);
}

#[test]
fn test_hir_ascii_class_bytes_xdigit() {
    let kind = &ast::ClassAsciiKind::Xdigit;
    let result = hir_ascii_class_bytes(kind);
    let expected_ranges = vec![
        hir::ClassBytesRange::new('0' as u8, '9' as u8),
        hir::ClassBytesRange::new('A' as u8, 'F' as u8),
        hir::ClassBytesRange::new('a' as u8, 'f' as u8),
    ];
    assert_eq!(result.set, hir::ClassBytes::new(expected_ranges).set);
}

