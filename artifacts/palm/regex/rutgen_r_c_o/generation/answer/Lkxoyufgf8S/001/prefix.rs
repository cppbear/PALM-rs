// Answer 0

#[test]
fn test_ascii_class_xdigit() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Xdigit);
}

#[test]
fn test_ascii_class_alnum() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Alnum);
}

#[test]
fn test_ascii_class_alpha() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Alpha);
}

#[test]
fn test_ascii_class_ascii() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Ascii);
}

#[test]
fn test_ascii_class_blank() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Blank);
}

#[test]
fn test_ascii_class_cntrl() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Cntrl);
}

#[test]
fn test_ascii_class_digit() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Digit);
}

#[test]
fn test_ascii_class_graph() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Graph);
}

#[test]
fn test_ascii_class_lower() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Lower);
}

#[test]
fn test_ascii_class_print() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Print);
}

#[test]
fn test_ascii_class_punct() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Punct);
}

#[test]
fn test_ascii_class_space() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Space);
}

#[test]
fn test_ascii_class_upper() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Upper);
}

#[test]
fn test_ascii_class_word() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Word);
}

