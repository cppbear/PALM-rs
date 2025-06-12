// Answer 0

#[test]
fn test_ascii_class_digit() {
    let kind = &ast::ClassAsciiKind::Digit;
    let result = ascii_class(kind);
}

#[test]
fn test_ascii_class_alnum() {
    let kind = &ast::ClassAsciiKind::Alnum;
    let result = ascii_class(kind);
}

#[test]
fn test_ascii_class_alpha() {
    let kind = &ast::ClassAsciiKind::Alpha;
    let result = ascii_class(kind);
}

#[test]
fn test_ascii_class_ascii() {
    let kind = &ast::ClassAsciiKind::Ascii;
    let result = ascii_class(kind);
}

#[test]
fn test_ascii_class_blank() {
    let kind = &ast::ClassAsciiKind::Blank;
    let result = ascii_class(kind);
}

#[test]
fn test_ascii_class_cntrl() {
    let kind = &ast::ClassAsciiKind::Cntrl;
    let result = ascii_class(kind);
}

#[test]
fn test_ascii_class_graph() {
    let kind = &ast::ClassAsciiKind::Graph;
    let result = ascii_class(kind);
}

#[test]
fn test_ascii_class_lower() {
    let kind = &ast::ClassAsciiKind::Lower;
    let result = ascii_class(kind);
}

#[test]
fn test_ascii_class_print() {
    let kind = &ast::ClassAsciiKind::Print;
    let result = ascii_class(kind);
}

#[test]
fn test_ascii_class_punct() {
    let kind = &ast::ClassAsciiKind::Punct;
    let result = ascii_class(kind);
}

#[test]
fn test_ascii_class_space() {
    let kind = &ast::ClassAsciiKind::Space;
    let result = ascii_class(kind);
}

#[test]
fn test_ascii_class_upper() {
    let kind = &ast::ClassAsciiKind::Upper;
    let result = ascii_class(kind);
}

#[test]
fn test_ascii_class_word() {
    let kind = &ast::ClassAsciiKind::Word;
    let result = ascii_class(kind);
}

#[test]
fn test_ascii_class_xdigit() {
    let kind = &ast::ClassAsciiKind::Xdigit;
    let result = ascii_class(kind);
}

