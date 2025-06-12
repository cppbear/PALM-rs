// Answer 0

#[test]
fn test_ascii_class_print() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Print;
    let result = ascii_class(&kind);
    let expected = &[(' ', '~')];

    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_alnum() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Alnum;
    let result = ascii_class(&kind);
    let expected = &[('0', '9'), ('A', 'Z'), ('a', 'z')];

    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_alpha() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Alpha;
    let result = ascii_class(&kind);
    let expected = &[('A', 'Z'), ('a', 'z')];

    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_ascii() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Ascii;
    let result = ascii_class(&kind);
    let expected = &[('\x00', '\x7F')];

    assert_eq!(result, expected);
} 

#[test]
fn test_ascii_class_blank() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Blank;
    let result = ascii_class(&kind);
    let expected = &[(' ', '\t')];

    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_cntrl() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Cntrl;
    let result = ascii_class(&kind);
    let expected = &[('\x00', '\x1F'), ('\x7F', '\x7F')];

    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_digit() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Digit;
    let result = ascii_class(&kind);
    let expected = &[('0', '9')];

    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_graph() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Graph;
    let result = ascii_class(&kind);
    let expected = &[('!', '~')];

    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_lower() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Lower;
    let result = ascii_class(&kind);
    let expected = &[('a', 'z')];

    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_punct() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Punct;
    let result = ascii_class(&kind);
    let expected = &[('!', '/'), (':', '@'), ('[', '`'), ('{', '~')];

    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_space() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Space;
    let result = ascii_class(&kind);
    let expected = &[
        ('\t', '\t'), ('\n', '\n'), ('\x0B', '\x0B'), ('\x0C', '\x0C'),
        ('\r', '\r'), (' ', ' '),
    ];

    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_upper() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Upper;
    let result = ascii_class(&kind);
    let expected = &[('A', 'Z')];

    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_word() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Word;
    let result = ascii_class(&kind);
    let expected = &[('0', '9'), ('A', 'Z'), ('_', '_'), ('a', 'z')];

    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_xdigit() {
    use ast::ClassAsciiKind;

    let kind = ClassAsciiKind::Xdigit;
    let result = ascii_class(&kind);
    let expected = &[('0', '9'), ('A', 'F'), ('a', 'f')];

    assert_eq!(result, expected);
}

