// Answer 0

#[test]
fn test_ascii_class_alnum() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Alnum);
    let expected: &[(char, char)] = &[('0', '9'), ('A', 'Z'), ('a', 'z')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_alpha() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Alpha);
    let expected: &[(char, char)] = &[('A', 'Z'), ('a', 'z')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_ascii() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Ascii);
    let expected: &[(char, char)] = &[('\x00', '\x7F')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_blank() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Blank);
    let expected: &[(char, char)] = &[(' ', '\t')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_cntrl() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Cntrl);
    let expected: &[(char, char)] = &[('\x00', '\x1F'), ('\x7F', '\x7F')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_digit() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Digit);
    let expected: &[(char, char)] = &[('0', '9')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_graph() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Graph);
    let expected: &[(char, char)] = &[('!', '~')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_lower() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Lower);
    let expected: &[(char, char)] = &[('a', 'z')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_print() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Print);
    let expected: &[(char, char)] = &[(' ', '~')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_punct() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Punct);
    let expected: &[(char, char)] = &[('!', '/'), (':', '@'), ('[', '`'), ('{', '~')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_space() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Space);
    let expected: &[(char, char)] = &[
        ('\t', '\t'),
        ('\n', '\n'),
        ('\x0B', '\x0B'),
        ('\x0C', '\x0C'),
        ('\r', '\r'),
        (' ', ' '),
    ];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_upper() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Upper);
    let expected: &[(char, char)] = &[('A', 'Z')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_word() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Word);
    let expected: &[(char, char)] = &[('0', '9'), ('A', 'Z'), ('_', '_'), ('a', 'z')];
    assert_eq!(result, expected);
}

#[test]
fn test_ascii_class_xdigit() {
    use ast::ClassAsciiKind;

    let result = ascii_class(&ClassAsciiKind::Xdigit);
    let expected: &[(char, char)] = &[('0', '9'), ('A', 'F'), ('a', 'f')];
    assert_eq!(result, expected);
}

