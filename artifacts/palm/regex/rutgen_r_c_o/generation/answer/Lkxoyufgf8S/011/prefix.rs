// Answer 0

#[test]
fn test_ascii_class_blank() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Blank;
    ascii_class(&kind);
}

#[test]
fn test_ascii_class_alnum() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Alnum;
    ascii_class(&kind);
}

#[test]
fn test_ascii_class_alpha() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Alpha;
    ascii_class(&kind);
}

#[test]
fn test_ascii_class_ascii() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Ascii;
    ascii_class(&kind);
}

#[test]
fn test_ascii_class_cntrl() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Cntrl;
    ascii_class(&kind);
}

#[test]
fn test_ascii_class_digit() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Digit;
    ascii_class(&kind);
}

#[test]
fn test_ascii_class_graph() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Graph;
    ascii_class(&kind);
}

#[test]
fn test_ascii_class_lower() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Lower;
    ascii_class(&kind);
}

#[test]
fn test_ascii_class_print() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Print;
    ascii_class(&kind);
}

#[test]
fn test_ascii_class_punct() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Punct;
    ascii_class(&kind);
}

#[test]
fn test_ascii_class_space() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Space;
    ascii_class(&kind);
}

#[test]
fn test_ascii_class_upper() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Upper;
    ascii_class(&kind);
}

#[test]
fn test_ascii_class_word() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Word;
    ascii_class(&kind);
}

#[test]
fn test_ascii_class_xdigit() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Xdigit;
    ascii_class(&kind);
}

