// Answer 0

#[test]
fn test_hir_ascii_class_bytes_alnum() {
    let kind = ast::ClassAsciiKind::Alnum;
    hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_alpha() {
    let kind = ast::ClassAsciiKind::Alpha;
    hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_ascii() {
    let kind = ast::ClassAsciiKind::Ascii;
    hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_blank() {
    let kind = ast::ClassAsciiKind::Blank;
    hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_cntrl() {
    let kind = ast::ClassAsciiKind::Cntrl;
    hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_digit() {
    let kind = ast::ClassAsciiKind::Digit;
    hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_graph() {
    let kind = ast::ClassAsciiKind::Graph;
    hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_lower() {
    let kind = ast::ClassAsciiKind::Lower;
    hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_print() {
    let kind = ast::ClassAsciiKind::Print;
    hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_punct() {
    let kind = ast::ClassAsciiKind::Punct;
    hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_space() {
    let kind = ast::ClassAsciiKind::Space;
    hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_upper() {
    let kind = ast::ClassAsciiKind::Upper;
    hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_word() {
    let kind = ast::ClassAsciiKind::Word;
    hir_ascii_class_bytes(&kind);
}

#[test]
fn test_hir_ascii_class_bytes_xdigit() {
    let kind = ast::ClassAsciiKind::Xdigit;
    hir_ascii_class_bytes(&kind);
}

